use std::{
    ops::{Div, Mul},
    sync::Arc,
};
use tracing::info;

use anyhow::{Context, Result};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::providers::Middleware;

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
}

impl<M: Middleware, N: Middleware> ProtectExecutor<M, N> {
    pub fn new(client: Arc<M>, sender_client: Arc<N>) -> Self {
        Self {
            client,
            sender_client,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempool> for ProtectExecutor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        info!("Executing tx");
        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .context("Error estimating gas usage: {}");
        info!("{:?}", gas_usage_result);
        let gas_usage = gas_usage_result?;

        // let gas_usage = self
        //     .client
        //     .estimate_gas(&action.tx, None)
        //     .await
        //     .context("Error estimating gas usage: {}")?;

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price = gas_bid_info.total_profit / gas_usage;
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price
                .mul(gas_bid_info.bid_percentage)
                .div(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
        }
        action.tx.set_gas_price(bid_gas_price);
        self.sender_client.send_transaction(action.tx, None).await?;
        Ok(())
    }
}
