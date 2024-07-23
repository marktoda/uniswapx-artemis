use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrder,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use uniswapx_rs::order::ResolvedOrder;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapXOrder(Box<UniswapXOrder>),
    PriorityOrder(Box<UniswapXOrder>),
    UniswapXRoute(Box<RoutedOrder>),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(SubmitTxToMempool),
}

/// Configuration for variables we need to pass to the strategy.
#[derive(Debug, Clone)]
pub struct Config {
    pub bid_percentage: u64,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenInTokenOut {
    pub token_in: String,
    pub token_out: String,
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Open(ResolvedOrder),
    Done,
}
