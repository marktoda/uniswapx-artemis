use super::{
    shared::{UniswapXStrategy, WETH_ADDRESS},
    types::{Config, OrderStatus, TokenInTokenOut},
};
use crate::collectors::{
    block_collector::NewBlock,
    uniswapx_order_collector::UniswapXOrder,
    uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
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
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use uniswapx_rs::order::{Order, OrderResolution, V2DutchOrder};

use super::types::{Action, Event};

const BLOCK_TIME: u64 = 12;
const DONE_EXPIRY: u64 = 300;
const REACTOR_ADDRESS: &str = "0x00000011F84B9aa48e5f8aA8B9897600006289Be";

#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXUniswapFill<M> {
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

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
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
            _ => None,
        }
    }
}

impl<M: Middleware + 'static> UniswapXStrategy<M> for UniswapXUniswapFill<M> {}

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
    fn decode_order(&self, encoded_order: &str) -> Result<V2DutchOrder, Box<dyn Error>> {
        let encoded_order = if encoded_order.starts_with("0x") {
            &encoded_order[2..]
        } else {
            encoded_order
        };
        let order_hex: Vec<u8> = hex::decode(encoded_order)?;

        Ok(V2DutchOrder::decode_inner(&order_hex, false)?)
    }

    // Process new orders as they come in.
    async fn process_order_event(&mut self, event: UniswapXOrder) -> Option<Action> {
        if self.last_block_timestamp == 0 {
            return None;
        }

        let order = self
            .decode_order(&event.encoded_order)
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
            let signed_orders = self.get_signed_orders(orders.clone()).ok()?;
            return Some(Action::SubmitTx(SubmitTxToMempool {
                tx: self
                    .build_fill(self.client.clone(), &self.executor_address, signed_orders, event)
                    .await
                    .ok()?,
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

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            match &batch.order {
                Order::V2DutchOrder(order) => {
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
                order_batch_data
                    .orders
                    .push(order_data.clone());
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
            match &order_data.order {
                Order::V2DutchOrder(order) => {
                    self.update_order_state(
                        order.clone(),
                        order_data.signature.clone(),
                        order_hash.clone().to_string(),
                    );
                },
                _ => {
                    error!("Invalid order type");
                }
            }
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

    fn update_order_state(&mut self, order: V2DutchOrder, signature: String, order_hash: String) {
        let resolved = order.resolve(self.last_block_timestamp + BLOCK_TIME);
        let order_status: OrderStatus = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
            _ => OrderStatus::Done
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
                        order: Order::V2DutchOrder(order),
                        hash: order_hash,
                        signature,
                        resolved: resolved_order,
                    },
                );
            }
            // Noop
            _ => {}
        }
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
