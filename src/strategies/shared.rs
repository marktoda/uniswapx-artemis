use crate::collectors::uniswapx_route_collector::RoutedOrder;
use anyhow::Result;
use async_trait::async_trait;
use bindings_uniswapx::{
    erc20::ERC20, shared_types::SignedOrder, swap_router_02_executor::SwapRouter02Executor,
};
use ethers::{
    abi::{ethabi, ParamType, Token},
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, H160, U256},
};
use std::sync::Arc;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

const REACTOR_ADDRESS: &str = "0x00000011F84B9aa48e5f8aA8B9897600006289Be";
const SWAPROUTER_02_ADDRESS: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const EXECUTOR_ADDRESS: &str = "0xa6b19B30593F6e70eabf6c05f9C96d66da65a0A1";

#[async_trait]
pub trait UniswapXStrategy<M: Middleware + 'static> {
    // builds a transaction to fill an order
    async fn build_fill(
        &self,
        client: Arc<M>,
        signed_orders: Vec<SignedOrder>,
        RoutedOrder { request, route }: RoutedOrder,
    ) -> Result<TypedTransaction> {
        let chain_id: U256 = client.get_chainid().await?;
        let fill_contract =
            SwapRouter02Executor::new(H160::from_str(EXECUTOR_ADDRESS)?, client.clone());

        let token_in: H160 = H160::from_str(&request.token_in)?;
        let token_out: H160 = H160::from_str(&request.token_out)?;

        let swaprouter_02_approval = self
            .get_tokens_to_approve(
                client.clone(),
                token_in,
                EXECUTOR_ADDRESS,
                SWAPROUTER_02_ADDRESS,
            )
            .await?;

        let reactor_approval = self
            .get_tokens_to_approve(client.clone(), token_out, EXECUTOR_ADDRESS, REACTOR_ADDRESS)
            .await?;

        // Strip off function selector
        let multicall_bytes = &route.method_parameters.calldata[10..];

        // Decode multicall into [Uint256, bytes[]] (deadline, multicallData)
        let decoded_multicall_bytes = ethabi::decode(
            &[
                ParamType::Uint(256),
                ParamType::Array(Box::new(ParamType::Bytes)),
            ],
            &Bytes::from_str(multicall_bytes)
                .ok()
                .expect("Failed to decode multicall bytes"),
        );

        let decoded_multicall_bytes = match decoded_multicall_bytes {
            Ok(data) => data[1].clone(), // already in bytes[]
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to decode multicall bytes: {}", e));
            }
        };

        // abi encode as [tokens to approve to swap router 02, tokens to approve to reactor,  multicall data]
        //               [address[], address[], bytes[]]
        let calldata = ethabi::encode(&[
            Token::Array(swaprouter_02_approval),
            Token::Array(reactor_approval),
            decoded_multicall_bytes,
        ]);
        let mut call = fill_contract.execute_batch(signed_orders, Bytes::from(calldata));
        Ok(call.tx.set_chain_id(chain_id.as_u64()).clone())
    }

    fn current_timestamp(&self) -> Result<u64> {
        let start = SystemTime::now();
        Ok(start.duration_since(UNIX_EPOCH)?.as_secs())
    }

    async fn get_tokens_to_approve(
        &self,
        client: Arc<M>,
        token: Address,
        from: &str,
        to: &str,
    ) -> Result<Vec<Token>, anyhow::Error> {
        if token == Address::zero() {
            return Ok(vec![]);
        }
        let token_contract = ERC20::new(token, client.clone());
        let allowance = token_contract
            .allowance(
                H160::from_str(from)
                    .ok()
                    .expect("Error encoding from address"),
                H160::from_str(to)
                    .ok()
                    .expect("Error encoding from address"),
            )
            .await
            .ok()
            .expect("Failed to get allowance");
        if allowance < U256::MAX / 2 {
            Ok(vec![Token::Address(token)])
        } else {
            Ok(vec![])
        }
    }
}
