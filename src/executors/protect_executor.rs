use std::{
    ops::{Div, Mul},
    sync::Arc,
};
use tracing::info;

use anyhow::Result;
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{
    middleware::MiddlewareBuilder,
    providers::Middleware,
    signers::{LocalWallet, Signer},
    types::U256,
};

use crate::strategies::keystore::KeyStore;

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
}

impl<M: Middleware, N: Middleware> ProtectExecutor<M, N> {
    pub fn new(client: Arc<M>, sender_client: Arc<N>, key_store: Arc<KeyStore>) -> Self {
        Self {
            client,
            sender_client,
            key_store,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempool> for ProtectExecutor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        // Acquire a key from the key store
        let (public_address, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("Acquired key: {}", public_address);

        let chain_id = u64::from_str_radix(
            &action
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

        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .unwrap_or_else(|err| {
                info!("Error estimating gas: {}", err);
                U256::from(1_000_000)
            });
        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result;

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
                .map_err(|err| anyhow::anyhow!("Error getting gas price: {}", err))?;
        }
        action.tx.set_gas_price(bid_gas_price);

        let sender_client = self.sender_client.clone();
        let nonce_manager = sender_client.nonce_manager(address);
        let signer = nonce_manager.with_signer(wallet);

        info!("Executing tx {:?}", action.tx);
        let result = signer
            .send_transaction(action.tx, None)
            .await
            .map_err(|err| anyhow::anyhow!("Error sending transaction: {}", err));

        match self.key_store.release_key(public_address.clone()).await {
            Ok(_) => {
                info!("Released key: {}", public_address);
            }
            Err(e) => {
                info!("Failed to release key: {}", e);
            }
        }
        // Send result up the stack
        result?;
        Ok(())
    }
}
