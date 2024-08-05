use crate::strategies::shared::EXECUTOR_ADDRESS;
use alloy_primitives::Uint;
use anyhow::Result;
use reqwest::header::ORIGIN;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;
use uniswapx_rs::order::{PriorityOrder, ResolvedOrder, V2DutchOrder};

use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::lock::Mutex;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;

const ROUTING_API: &str = "https://api.uniswap.org/v1/quote";
const SLIPPAGE_TOLERANCE: &str = "0.5";
const DEADLINE: u64 = 1000;

#[derive(Debug, Clone)]
pub struct V2DutchOrderData {
    pub order: V2DutchOrder,
    pub hash: String,
    pub signature: String,
    pub resolved: ResolvedOrder,
}

#[derive(Debug, Clone)]
pub struct PriorityOrderData {
    pub order: PriorityOrder,
    pub hash: String,
    pub signature: String,
    pub resolved: ResolvedOrder,
}

#[derive(Debug, Clone)]
pub enum OrderData {
    V2DutchOrderData(V2DutchOrderData),
    PriorityOrderData(PriorityOrderData),
}

impl OrderData {
    pub fn signature(&self) -> String {
        match self {
            OrderData::V2DutchOrderData(data) => data.signature.clone(),
            OrderData::PriorityOrderData(data) => data.signature.clone(),
        }
    }

    pub fn hash(&self) -> String {
        match self {
            OrderData::V2DutchOrderData(data) => data.hash.clone(),
            OrderData::PriorityOrderData(data) => data.hash.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderBatchData {
    pub orders: Vec<OrderData>,
    pub amount_in: Uint<256, 4>,
    pub amount_out_required: Uint<256, 4>,
    pub token_in: String,
    pub token_out: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
enum TradeType {
    #[serde(rename = "exactIn")]
    ExactIn,
    #[serde(rename = "exactOut")]
    ExactOut,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RoutingApiQuery {
    token_in_address: String,
    token_out_address: String,
    token_in_chain_id: u64,
    token_out_chain_id: u64,
    #[serde(rename = "type")]
    trade_type: TradeType,
    amount: String,
    recipient: String,
    slippage_tolerance: String,
    deadline: u64,
    #[serde(rename = "enableUniversalRouter")]
    enable_universal_router: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TokenInRoute {
    address: String,
    chain_id: u64,
    symbol: String,
    decimals: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct V3Route {
    address: String,
    token_in: TokenInRoute,
    token_out: TokenInRoute,
    fee: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct V2Route {
    address: String,
    token_in: TokenInRoute,
    token_out: TokenInRoute,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Route {
    #[serde(rename = "v3-pool")]
    V3(V3Route),
    #[serde(rename = "v2-pool")]
    V2(V2Route),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub struct MethodParameters {
    pub calldata: String,
    pub value: String,
    pub to: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRoute {
    pub quote: String,
    pub quote_gas_adjusted: String,
    pub gas_price_wei: String,
    pub gas_use_estimate_quote: String,
    pub gas_use_estimate: String,
    pub route: Vec<Vec<Route>>,
    pub method_parameters: MethodParameters,
}

pub struct RouteOrderParams {
    pub chain_id: u64,
    pub token_in: String,
    pub token_out: String,
    pub amount: String,
}

#[derive(Clone, Debug)]
pub struct RoutedOrder {
    pub route: OrderRoute,
    pub request: OrderBatchData,
}

/// A new order event, containing the internal order.
#[derive(Debug, Clone, Deserialize)]
pub struct RouteResponse {
    pub orders: Vec<Route>,
}

/// A collector that listens for new orders on UniswapX, and generates a stream of
/// [events](Route) which contain the order.
pub struct UniswapXRouteCollector {
    pub client: Client,
    pub chain_id: u64,
    pub route_request_receiver: Mutex<Receiver<Vec<OrderBatchData>>>,
    pub route_sender: Sender<RoutedOrder>,
}

impl UniswapXRouteCollector {
    pub fn new(
        chain_id: u64,
        route_request_receiver: Receiver<Vec<OrderBatchData>>,
        route_sender: Sender<RoutedOrder>,
    ) -> Self {
        Self {
            client: Client::new(),
            chain_id,
            route_request_receiver: Mutex::new(route_request_receiver),
            route_sender,
        }
    }
}

/// Implementation of the [Collector](Collector) trait for the
/// [UniswapXRouteCollector](UniswapXRouteCollector).
// TODO: implement order deduplication
#[async_trait]
impl Collector<RoutedOrder> for UniswapXRouteCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, RoutedOrder>> {
        let stream = async_stream::stream! {
            let mut receiver = self.route_request_receiver.lock().await;
            while let Some(route_requests) = receiver.recv().await {
                let tasks: FuturesUnordered<_> = route_requests.iter()
                    .map(|batch| {
                        let OrderBatchData { orders, token_in, token_out, amount_in, .. } = batch.clone();
                        info!(
                            "Checking batch: {} orders, token in: {}, token out: {}",
                            orders.len(), token_in, token_out
                        );

                        async move {
                            (batch, route_order(RouteOrderParams {
                                chain_id: self.chain_id,
                                token_in: token_in.clone(),
                                token_out: token_out.clone(),
                                amount: amount_in.to_string(),
                            }).await)
                        }
                    }).collect();

                let routes: Vec<_> = tasks.collect().await;
                for (batch, route_result) in routes {
                    if let Ok(route) = route_result {
                        yield RoutedOrder {
                            request: batch.clone(),
                            route: route,
                        };
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}

pub async fn route_order(params: RouteOrderParams) -> Result<OrderRoute> {
    // TODO: support exactOutput
    let query = RoutingApiQuery {
        token_in_address: resolve_address(params.token_in),
        token_out_address: resolve_address(params.token_out),
        token_in_chain_id: params.chain_id,
        token_out_chain_id: params.chain_id,
        trade_type: TradeType::ExactIn,
        amount: params.amount,
        recipient: EXECUTOR_ADDRESS.to_string(),
        slippage_tolerance: SLIPPAGE_TOLERANCE.to_string(),
        enable_universal_router: false,
        deadline: DEADLINE,
    };

    let query_string = serde_qs::to_string(&query).unwrap();

    let client = reqwest::Client::new();

    Ok(client
        .get(format!("{}?{}", ROUTING_API, query_string))
        .header(ORIGIN, "https://app.uniswap.org")
        .header("x-request-source", "uniswap-web")
        .send()
        .await?
        .json::<OrderRoute>()
        .await?)
}

// our routing provider requires that "ETH" be used instead of the zero address
fn resolve_address(token: String) -> String {
    if token == "0x0000000000000000000000000000000000000000" {
        return "ETH".to_string();
    }
    return token;
}
