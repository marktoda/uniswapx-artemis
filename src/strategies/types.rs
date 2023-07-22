use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrder,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapXOrder(Box<UniswapXOrder>),
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
