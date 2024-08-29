use super::{
    shared::UniswapXStrategy,
    types::{Config, OrderStatus},
};
use crate::{
    collectors::{
        block_collector::NewBlock,
        uniswapx_order_collector::UniswapXOrder,
        uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
    },
    strategies::types::SubmitTxToMempoolWithExecutionMetadata,
};
use alloy_primitives::Uint;
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_uniswapx::shared_types::SignedOrder;
use ethers::{
    providers::Middleware,
    types::{Address, Bytes, Filter, U256},
    utils::hex,
};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use uniswapx_rs::order::{Order, OrderResolution, PriorityOrder, MPS};

use super::types::{Action, Event};

const BLOCK_TIME: u64 = 2;
const DONE_EXPIRY: u64 = 300;
// Base addresses
const REACTOR_ADDRESS: &str = "0x000000001Ec5656dcdB24D90DFa42742738De729";
pub const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";

#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    // amount of quote token we can get
    quote: U256,
    // amount of quote token needed to fill the order
    amount_out_required: U256,
}

impl ExecutionMetadata {
    pub fn new(quote: U256, amount_out_required: U256) -> Self {
        Self {
            quote,
            amount_out_required,
        }
    }

    pub fn calculate_priority_fee(&self, bid_percentage: u64) -> Option<U256> {
        if self.quote.le(&self.amount_out_required) {
            return None;
        }

        let profit_quote = self.quote.saturating_sub(self.amount_out_required);

        let mps_of_improvement = profit_quote
            .saturating_mul(U256::from(MPS))
            .checked_div(self.amount_out_required)?;
        info!("mps_of_improvement: {}", mps_of_improvement);
        let priority_fee = mps_of_improvement
            .checked_mul(U256::from(bid_percentage))?
            .checked_div(U256::from(100))?;
        return Some(priority_fee);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXPriorityFill<M> {
    /// Ethers client.
    client: Arc<M>,
    /// executor address
    executor_address: String,
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

impl<M: Middleware + 'static> UniswapXPriorityFill<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
    ) -> Self {
        info!("syncing state");

        Self {
            client,
            executor_address: config.executor_address,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            last_block_timestamp: 0,
            open_orders: HashMap::new(),
            done_orders: HashMap::new(),
            batch_sender: sender,
            route_receiver: receiver,
        }
    }

    fn open_orders(&self) -> &HashMap<String, OrderData> {
        &self.open_orders
    }

    fn open_orders_mut(&mut self) -> &mut HashMap<String, OrderData> {
        &mut self.open_orders
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UniswapXPriorityFill<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(&order).await,
            Event::NewBlock(block) => self.process_new_block_event(&block).await,
            Event::UniswapXRoute(route) => self.process_new_route(&route).await,
        }
    }
}

impl<M: Middleware + 'static> UniswapXStrategy<M> for UniswapXPriorityFill<M> {}

impl<M: Middleware + 'static> UniswapXPriorityFill<M> {
    fn decode_order(&self, encoded_order: &str) -> Result<PriorityOrder, Box<dyn Error>> {
        let encoded_order = if encoded_order.starts_with("0x") {
            &encoded_order[2..]
        } else {
            encoded_order
        };
        let order_hex = hex::decode(encoded_order)?;

        Ok(PriorityOrder::decode_inner(&order_hex, false)?)
    }

    async fn process_order_event(&mut self, event: &UniswapXOrder) -> Option<Action> {
        if self.last_block_timestamp == 0 {
            return None;
        }

        let order = self
            .decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok()?;

        self.update_order_state(&order, &event.signature, &event.order_hash);
        self.send_order_if_open(&event.order_hash)
            .await
            .map_err(|e| error!("failed to send order: {}", e))
            .ok()?;

        None
    }

    async fn process_new_route(&mut self, event: &RoutedOrder) -> Option<Action> {
        if event
            .request
            .orders
            .iter()
            .any(|o: &OrderData| self.done_orders.contains_key(&o.hash))
        {
            info!("Skipping route with done orders");
            return None;
        }

        let OrderBatchData {
            // orders,
            orders,
            amount_out_required,
            ..
        } = &event.request;

        if let Some(profit) = self.get_execution_metadata(&event) {
            info!(
                "Sending trade: num trades: {} routed quote: {}, batch needs: {}",
                orders.len(),
                event.route.quote,
                amount_out_required,
            );

            let signed_orders = self.get_signed_orders(orders.clone()).ok()?;
            return Some(Action::SubmitPublicTx(
                SubmitTxToMempoolWithExecutionMetadata {
                    execution: SubmitTxToMempool {
                        tx: self
                            .build_fill(
                                self.client.clone(),
                                &self.executor_address,
                                signed_orders,
                                event,
                            )
                            .await
                            .ok()?,
                        gas_bid_info: Some(GasBidInfo {
                            bid_percentage: self.bid_percentage,
                            // this field is not used for priority orders
                            total_profit: U256::from(0),
                        }),
                    },
                    metadata: profit,
                },
            ));
        }

        None
    }

    /// Process new block events, updating the internal state.
    async fn process_new_block_event(&mut self, event: &NewBlock) -> Option<Action> {
        self.last_block_number = event.number.as_u64();
        self.last_block_timestamp = event.timestamp.as_u64();

        info!(
            "Processing block {} at {}, Order set sizes -- open: {}, done: {}",
            event.number,
            event.timestamp,
            self.open_orders().len(),
            self.done_orders.len()
        );
        self.handle_fills()
            .await
            .map_err(|e| error!("Error handling fills {}", e))
            .ok()?;
        self.update_open_orders().await;
        self.prune_done_orders();

        None
    }

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            match &batch.order {
                Order::PriorityOrder(order) => {
                    signed_orders.push(SignedOrder {
                        order: Bytes::from(order.encode_inner()),
                        sig: Bytes::from_str(&batch.signature)?,
                    });
                }
                _ => {
                    return Err(anyhow::anyhow!("Invalid order type"));
                }
            }
        }
        Ok(signed_orders)
    }

    fn get_order_batch(&self, order_data: &OrderData) -> OrderBatchData {
        let amount_in: Uint<256, 4> = order_data.resolved.input.amount;
        let amount_out = order_data
            .resolved
            .outputs
            .iter()
            .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount));

        OrderBatchData {
            orders: vec![order_data.clone()],
            amount_in,
            amount_out_required: amount_out,
            token_in: order_data.resolved.input.token.clone(),
            token_out: order_data.resolved.outputs[0].token.clone(),
        }
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
            self.open_orders_mut().remove(&order_hash);
            // add to done
            self.done_orders.insert(
                order_hash.to_string(),
                self.current_timestamp()? + DONE_EXPIRY,
            );
        }

        Ok(())
    }

    /// The profit of a priority order is calculated a bit differently
    /// Rationale:
    ///     - we will always bid the base fee
    ///     - since we have to provide 1 MP (1/1000th of a bp) for every wei of priority fee
    ///     - we return the data needed to calculate the maximum MPS of improvement we can offer from our quote and the order specs
    fn get_execution_metadata(
        &self,
        RoutedOrder { request, route }: &RoutedOrder,
    ) -> Option<ExecutionMetadata> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            return None;
        }

        return Some({
            ExecutionMetadata {
                quote,
                amount_out_required,
            }
        });
    }

    // Resolve and order and update its state
    fn update_order_state(
        &mut self,
        order: &PriorityOrder,
        signature: &String,
        order_hash: &String,
    ) {
        let resolved = order.resolve(
            self.last_block_number,
            self.last_block_timestamp + BLOCK_TIME,
            Uint::from(0),
        );
        let order_status: OrderStatus = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::NotFillableYet => OrderStatus::NotFillableYet,
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
        };

        match order_status {
            OrderStatus::Done => {
                self.mark_as_done(&order_hash);
            }
            OrderStatus::NotFillableYet => {
                info!("Order not fillable yet, skipping: {}", order_hash);
            }
            OrderStatus::Open(resolved_order) => {
                if self.done_orders.contains_key(order_hash) {
                    info!("Order already done, skipping: {}", order_hash);
                    return;
                }
                if self.open_orders().contains_key(order_hash) {
                    let existing_order = self.open_orders_mut().get_mut(order_hash).unwrap();
                    info!("Updating order {}", order_hash);
                    existing_order.resolved = resolved_order;
                } else {
                    info!("Adding new order {}", order_hash);
                    self.open_orders_mut().insert(
                        order_hash.clone(),
                        OrderData {
                            order: Order::PriorityOrder(order.clone()),
                            hash: order_hash.clone(),
                            signature: signature.clone(),
                            resolved: resolved_order,
                        },
                    );
                }
            }
        }
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

    async fn update_open_orders(&mut self) {
        // TODO: this is nasty, plz cleanup
        let binding = self.open_orders.clone();
        let order_hashes: Vec<(&String, &OrderData)> = binding.iter().collect();
        for (order_hash, order_data) in order_hashes {
            match &order_data.order {
                Order::PriorityOrder(order) => {
                    self.update_order_state(&order, &order_data.signature, &order_hash.to_string());
                    self.send_order_if_open(order_hash)
                        .await
                        .map_err(|e| error!("failed to send order: {}", e))
                        .ok();
                }
                _ => {
                    error!("Invalid order type");
                }
            }
        }
    }

    async fn send_order_if_open(&self, order_hash: &String) -> Result<()> {
        if let Some(order_data) = self.open_orders().get(order_hash) {
            let order_batch = self.get_order_batch(order_data);
            self.batch_sender.send(vec![order_batch]).await?;
        }
        Ok(())
    }

    fn mark_as_done(&mut self, order: &str) {
        if self.open_orders().contains_key(order) {
            self.open_orders_mut().remove(order);
        }
        if !self.done_orders.contains_key(order) {
            self.done_orders
                .insert(order.to_string(), self.last_block_timestamp + DONE_EXPIRY);
        }
    }
}
