use super::types::Config;
use crate::collectors::{
    block_collector::NewBlock,
    uniswapx_order_collector::{UniswapXOrder, CHAIN_ID},
    uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
};
use alloy_primitives::Uint;
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_uniswapx::{
    swap_router_02_executor::SwapRouter02Executor, shared_types::SignedOrder,
    erc20::ERC20,
};
use ethers::{
    abi::{ethabi, AbiEncode, Token},
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, Filter, H160, U256},
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use uniswapx_rs::order::{
    decode_order, encode_order, ExclusiveDutchOrder, OrderResolution, ResolvedOrder,
};
use std::env;

use super::types::{Action, Event};

const BLOCK_TIME: u64 = 12;
const DONE_EXPIRY: u64 = 300;
const REACTOR_ADDRESS: &str = "0x6000da47483062A0D734Ba3dc7576Ce6A0B645C4"; 
pub const SWAPROUTER_02_ADDRESS: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const EXECUTOR_ADDRESS: &str = env!("EXECUTOR_ADDRESS", "$EXECUTOR_ADDRESS is not set");

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct TokenInTokenOut {
    token_in: String,
    token_out: String,
}

#[derive(Debug, Clone)]
enum OrderStatus {
    Open(ResolvedOrder),
    Done,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXUniswapFill<M> {
    /// Ethers client.
    client: Arc<M>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    last_block_timestamp: u64,
    // map of open order hashes to order data
    open_orders: HashMap<String, OrderData>,
    // map of done order hashes to time at which we can safely prune them
    done_orders: HashMap<String, u64>,
    batch_sender: Sender<Vec<OrderBatchData>>,
    route_receiver: Receiver<RoutedOrder>,
}

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
    ) -> Self {
        info!("syncing state");
        info!("EXECUTOR_ADDRESS: {}", EXECUTOR_ADDRESS);

        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            last_block_timestamp: 0,
            open_orders: HashMap::new(),
            done_orders: HashMap::new(),
            batch_sender: sender,
            route_receiver: receiver,
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UniswapXUniswapFill<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(*order).await,
            Event::NewBlock(block) => self.process_new_block_event(block).await,
            Event::UniswapXRoute(route) => self.process_new_route(*route).await,
        }
    }
}

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
    // Process new orders as they come in.
    async fn process_order_event(&mut self, event: UniswapXOrder) -> Option<Action> {
        if self.last_block_timestamp == 0 {
            return None;
        }

        let order = decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok()?;

        self.update_order_state(order, event.signature, event.order_hash);
        None
    }

    async fn process_new_route(&mut self, event: RoutedOrder) -> Option<Action> {
        if event
            .request
            .orders
            .iter()
            .any(|o| self.done_orders.contains_key(&o.hash))
        {
            return None;
        }

        let OrderBatchData {
            // orders,
            orders,
            amount_out_required,
            ..
        } = &event.request;

        if let Some(profit) = self.get_profit_eth(&event) {
            info!(
                "Sending trade: num trades: {} routed quote: {}, batch needs: {}, profit: {} wei",
                orders.len(),
                event.route.quote_gas_adjusted,
                amount_out_required,
                profit
            );

            return Some(Action::SubmitTx(SubmitTxToMempool {
                tx: self.build_fill(event).await.ok().unwrap(),
                gas_bid_info: Some(GasBidInfo {
                    bid_percentage: self.bid_percentage,
                    total_profit: profit,
                }),
            }));
        }

        None
    }

    /// Process new block events, updating the internal state.
    async fn process_new_block_event(&mut self, event: NewBlock) -> Option<Action> {
        self.last_block_number = event.number.as_u64();
        self.last_block_timestamp = event.timestamp.as_u64();

        info!(
            "Processing block {} at {}, Order set sizes -- open: {}, done: {}",
            event.number,
            event.timestamp,
            self.open_orders.len(),
            self.done_orders.len()
        );
        self.handle_fills()
            .await
            .map_err(|e| error!("Error handling fills {}", e))
            .ok()?;
        self.update_open_orders();
        self.prune_done_orders();

        self.batch_sender
            .send(self.get_order_batches().values().cloned().collect())
            .await
            .ok()?;

        None
    }

    // builds a transaction to fill an order
    async fn build_fill(&self, RoutedOrder { request, route }: RoutedOrder) -> Result<TypedTransaction> {
        let fill_contract =
            SwapRouter02Executor::new(H160::from_str(EXECUTOR_ADDRESS)?, self.client.clone());
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in request.orders.iter() {
            let OrderData {
                order, signature, ..
            } = batch;
            signed_orders.push(SignedOrder {
                order: Bytes::from(encode_order(order)),
                sig: Bytes::from_str(signature)?,
            });
        }
        // slice off function selector
        let cleaned_method_paramaters = &route.method_parameters.calldata[10..];
        info!("cleaned_method_paramaters: {}", cleaned_method_paramaters);

        let mut tokens_to_approve_to_swap_router02 = Vec::new();
        let mut tokens_to_approve_to_reactor = Vec::new();
        // check token approvals
        let token_in_contract = ERC20::new(H160::from_str(&request.token_in)?, self.client.clone());
        let token_out_contract = ERC20::new(H160::from_str(&request.token_out)?, self.client.clone());
        let allowance_in = token_in_contract
            .allowance(
                H160::from_str(EXECUTOR_ADDRESS)?,
                H160::from_str(SWAPROUTER_02_ADDRESS)?,
            )
            .await?;
        if allowance_in < U256::MAX / 2 {
            tokens_to_approve_to_swap_router02.push(Token::Address(H160::from_str(&request.token_in)?));
        }
        let allowance_out = token_out_contract
            .allowance(
                H160::from_str(EXECUTOR_ADDRESS)?,
                H160::from_str(REACTOR_ADDRESS)?,
            )
            .await?;
        if allowance_out < U256::MAX / 2 {
            tokens_to_approve_to_reactor.push(Token::Address(H160::from_str(&request.token_out)?));
        }
        // abi encode as [tokensToApproveForSwapRouter02, tokensToApproveForReactor, multicall data]
        let calldata = ethabi::encode(&[
            Token::Array(tokens_to_approve_to_swap_router02),
            // TODO: fix for multiple outputs
            Token::Array(tokens_to_approve_to_reactor),
            Token::Bytes(Bytes::from_str(&cleaned_method_paramaters)?.encode()),
        ]);
        let mut call = fill_contract.execute_batch(
            signed_orders,
            Bytes::from(calldata),
        );

        Ok(call.tx.set_chain_id(CHAIN_ID).clone())
    }

    fn get_order_batches(&self) -> HashMap<TokenInTokenOut, OrderBatchData> {
        let mut order_batches: HashMap<TokenInTokenOut, OrderBatchData> = HashMap::new();

        // group orders by token in and token out
        self.open_orders.iter().for_each(|(_, order_data)| {
            let token_in_token_out = TokenInTokenOut {
                token_in: order_data.resolved.input.token.clone(),
                token_out: order_data.resolved.outputs[0].token.clone(),
            };

            let amount_in = order_data.resolved.input.amount;
            let amount_out = order_data
                .resolved
                .outputs
                .iter()
                .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount));

            // insert new order and update total amount out
            if let std::collections::hash_map::Entry::Vacant(e) =
                order_batches.entry(token_in_token_out.clone())
            {
                e.insert(OrderBatchData {
                    orders: vec![order_data.clone()],
                    amount_in,
                    amount_out_required: amount_out,
                    token_in: order_data.resolved.input.token.clone(),
                    token_out: order_data.resolved.outputs[0].token.clone(),
                });
            } else {
                let order_batch_data = order_batches.get_mut(&token_in_token_out).unwrap();
                order_batch_data.orders.push(order_data.clone());
                order_batch_data.amount_in = order_batch_data.amount_in.wrapping_add(amount_in);
                order_batch_data.amount_out_required = order_batch_data
                    .amount_out_required
                    .wrapping_add(amount_out);
            }
        });
        order_batches
    }

    async fn handle_fills(&mut self) -> Result<()> {
        let reactor_address = REACTOR_ADDRESS.parse::<Address>().unwrap();
        let filter = Filter::new()
            .select(self.last_block_number)
            .address(reactor_address)
            .event("Fill(bytes32,address,address,uint256)");

        // early return on error
        let logs = self.client.get_logs(&filter).await?;
        for log in logs {
            let order_hash = format!("0x{:x}", log.topics[1]);
            // remove from open
            info!("Removing filled order {}", order_hash);
            self.open_orders.remove(&order_hash);
            // add to done
            self.done_orders.insert(
                order_hash.to_string(),
                self.current_timestamp()? + DONE_EXPIRY,
            );
        }

        Ok(())
    }

    fn prune_done_orders(&mut self) {
        let mut to_remove = Vec::new();
        for (order_hash, deadline) in self.done_orders.iter() {
            if *deadline < self.last_block_timestamp {
                to_remove.push(order_hash.clone());
            }
        }
        for order_hash in to_remove {
            self.done_orders.remove(&order_hash);
        }
    }

    fn update_open_orders(&mut self) {
        // TODO: this is nasty, plz cleanup
        let binding = self.open_orders.clone();
        let order_hashes: Vec<(&String, &OrderData)> = binding.iter().collect();
        for (order_hash, order_data) in order_hashes {
            self.update_order_state(
                order_data.order.clone(),
                order_data.signature.clone(),
                order_hash.clone().to_string(),
            );
        }
    }

    fn mark_as_done(&mut self, order: &str) {
        if self.open_orders.contains_key(order) {
            self.open_orders.remove(order);
        }
        if !self.done_orders.contains_key(order) {
            self.done_orders
                .insert(order.to_string(), self.last_block_timestamp + DONE_EXPIRY);
        }
    }

    fn update_order_state(
        &mut self,
        order: ExclusiveDutchOrder,
        signature: String,
        order_hash: String,
    ) {
        let resolved = order.resolve(self.last_block_timestamp + BLOCK_TIME);
        let order_status: OrderStatus = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
        };

        match order_status {
            OrderStatus::Done => {
                self.mark_as_done(&order_hash);
            }
            OrderStatus::Open(resolved_order) => {
                if self.done_orders.contains_key(&order_hash) {
                    info!("Order already done, skipping: {}", order_hash);
                    return;
                }
                if !self.open_orders.contains_key(&order_hash) {
                    info!("Adding new order {}", order_hash);
                }
                self.open_orders.insert(
                    order_hash.clone(),
                    OrderData {
                        order,
                        hash: order_hash,
                        signature,
                        resolved: resolved_order,
                    },
                );
            }
        }
    }

    fn current_timestamp(&self) -> Result<u64> {
        let start = SystemTime::now();
        Ok(start.duration_since(UNIX_EPOCH)?.as_secs())
    }

    fn get_profit_eth(&self, RoutedOrder { request, route }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            return None;
        }
        let profit_quote = quote.saturating_sub(amount_out_required);

        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(profit_quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        profit_quote
            .saturating_mul(gas_use_eth)
            .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
    }
}
