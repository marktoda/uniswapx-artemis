use std::sync::Arc;
use tracing::info;

use anyhow::{Context, Result};
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{providers::Middleware, types::U256};

use crate::strategies::types::SubmitTxToMempoolWithExecutionMetadata;

/// An executor that sends transactions to the public mempool.
pub struct Public1559Executor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
}

impl<M: Middleware, N: Middleware> Public1559Executor<M, N> {
    pub fn new(client: Arc<M>, sender_client: Arc<N>) -> Self {
        Self {
            client,
            sender_client,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempoolWithExecutionMetadata> for Public1559Executor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempoolWithExecutionMetadata) -> Result<()> {
        let gas_usage_result = self
            .client
            .estimate_gas(&action.execution.tx, None)
            .await
            .unwrap_or_else(|err| {
                info!("Error estimating gas: {}", err);
                U256::from(1_000_000)
            });
        info!("Gas Usage {:?}", gas_usage_result);

        let bid_priority_fee;
        let base_fee: U256 = self
            .client
            .get_gas_price()
            .await
            .context("Error getting gas price: {}")?;

        if let Some(gas_bid_info) = action.execution.gas_bid_info {
            // priority fee at which we'd break even, meaning 100% of profit goes to user in the form of price improvement
            // TODO: use gas estimate here
            bid_priority_fee = action
                .metadata
                .calculate_priority_fee(gas_bid_info.bid_percentage)
        } else {
            bid_priority_fee = Some(U256::from(50));
        }

        let eip1559_tx = action.execution.tx.as_eip1559_mut();
        if let Some(eip1559_tx) = eip1559_tx {
            eip1559_tx.max_fee_per_gas = Some(base_fee);
            eip1559_tx.max_priority_fee_per_gas = bid_priority_fee;
        } else {
            return Err(anyhow::anyhow!("Transaction is not EIP1559"));
        }

        action.execution.tx.set_gas(gas_usage_result);

        info!("Executing tx {:?}", action.execution.tx);
        self.sender_client
            .send_transaction(action.execution.tx, None)
            .await
            .map_err(|err| anyhow::anyhow!("Error sending transaction: {}", err))?;
        Ok(())
    }
}
