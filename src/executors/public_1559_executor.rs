use std::sync::Arc;
use serde_json::Value;
use tracing::info;

use anyhow::{Context, Result};
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{
    middleware::MiddlewareBuilder,
    providers::{Middleware, MiddlewareError},
    signers::{LocalWallet, Signer},
    types::U256,
};

use crate::constants::ReactorErrorCode;
use crate::strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata};

/// An executor that sends transactions to the public mempool.
pub struct Public1559Executor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
}

impl<M: Middleware, N: Middleware> Public1559Executor<M, N> {
    pub fn new(client: Arc<M>, sender_client: Arc<N>, key_store: Arc<KeyStore>) -> Self {
        Self {
            client,
            sender_client,
            key_store,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempoolWithExecutionMetadata> for Public1559Executor<M, N>
where
    M: Middleware + 'static,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempoolWithExecutionMetadata) -> Result<()> {
        let order_hash = action.metadata.order_hash.clone();
        // Acquire a key from the key store
        let (public_address, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("Acquired key: {}", public_address);

        let chain_id = u64::from_str_radix(
            &action
                .execution
                .tx
                .chain_id()
                .expect("Chain ID not found on transaction")
                .to_string(),
            10,
        )
        .expect("Failed to parse chain ID");

        let wallet: LocalWallet = private_key
            .as_str()
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let address = wallet.address();

        action.execution.tx.set_from(address);

        // redundant match to log specific reasons
        // always use 1_000_000 gas for now
        let gas_usage_result = self
            .client
            .estimate_gas(&action.execution.tx, None)
            .await
            .unwrap_or_else(|err| {
                if let Some(Value::String(four_byte)) = err.as_error_response().unwrap().data.clone() {
                    match four_byte.into() {
                        ReactorErrorCode::OrderNotFillable => {
                            info!("{} - Estimating gas but OrderNotFillable yet", order_hash);
                            return U256::from(1_000_000);
                        }
                        _ => {
                            return U256::from(1_000_000);
                        }
                    }
                }
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

        let sender_client = self.sender_client.clone();
        let nonce_manager = sender_client.nonce_manager(address);
        let signer = nonce_manager.with_signer(wallet);

        info!("Executing tx {:?}", action.execution.tx);
        let result = signer.send_transaction(action.execution.tx, None).await;

        // Block on pending transaction getting confirmations
        match result {
            Ok(tx) => {
                let receipt = tx.confirmations(1)
                    .await
                    .map_err(|e| anyhow::anyhow!("Error waiting for confirmations: {}", e))?;
                info!("{} - receipt: {:?}", action.metadata.order_hash, receipt);
            }
            Err(e) => {
                info!("Error sending transaction: {}", e);
            }
        }

        // regardless of outcome, ensure we release the key
        match self.key_store.release_key(public_address.clone()).await {
            Ok(_) => {
                info!("Released key: {}", public_address);
            }
            Err(e) => {
                info!("Failed to release key: {}", e);
            }
        }
        Ok(())
    }
}
