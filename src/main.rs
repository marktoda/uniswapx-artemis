use anyhow::Result;
use clap::Parser;

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::uniswapx_order_collector::OrderType;
use collectors::{
    block_collector::BlockCollector, uniswapx_order_collector::UniswapXOrderCollector,
    uniswapx_route_collector::UniswapXRouteCollector,
};
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Http, Provider, Ws},
    signers::{LocalWallet, Signer},
};
use executors::protect_executor::ProtectExecutor;
use executors::public_1559_executor::Public1559Executor;
use std::sync::Arc;
use strategies::priority_strategy::UniswapXPriorityFill;
use strategies::{
    types::{Action, Config, Event},
    uniswapx_strategy::UniswapXUniswapFill,
};
use tokio::sync::mpsc::channel;
use tracing::{error, info, Level};
use tracing_subscriber::{filter, prelude::*};

pub mod collectors;
pub mod executors;
pub mod strategies;

const MEV_BLOCKER: &str = "https://rpc.mevblocker.io/noreverts";

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    /// Percentage of profit to pay in gas.
    #[arg(long)]
    pub bid_percentage: u64,

    /// Private key for sending txs.
    #[arg(long)]
    pub executor_address: String,

    /// Order type to use.
    #[arg(long)]
    pub order_type: OrderType,

    /// chain id
    #[arg(long)]
    pub chain_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("uniswapx_artemis", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);
    let chain_id = args.chain_id;

    let mevblocker_provider =
        Provider::<Http>::try_from(MEV_BLOCKER).expect("could not instantiate HTTP Provider");

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));
    let mevblocker_provider = Arc::new(
        mevblocker_provider
            .nonce_manager(address)
            .with_signer(wallet),
    );

    // Set up engine.
    let mut engine = Engine::default();

    // Set up block collector.
    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    let (batch_sender, batch_receiver) = channel(512);
    let (route_sender, route_receiver) = channel(512);

    let uniswapx_order_collector = Box::new(UniswapXOrderCollector::new(
        chain_id,
        args.order_type.clone(),
    ));
    let uniswapx_order_collector = CollectorMap::new(uniswapx_order_collector, |e| {
        Event::UniswapXOrder(Box::new(e))
    });
    engine.add_collector(Box::new(uniswapx_order_collector));

    let uniswapx_route_collector = Box::new(UniswapXRouteCollector::new(
        chain_id,
        batch_receiver,
        route_sender,
        args.executor_address.clone(),
    ));
    let uniswapx_route_collector = CollectorMap::new(uniswapx_route_collector, |e| {
        Event::UniswapXRoute(Box::new(e))
    });
    engine.add_collector(Box::new(uniswapx_route_collector));

    let config = Config {
        bid_percentage: args.bid_percentage,
        executor_address: args.executor_address,
    };

    match &args.order_type {
        OrderType::DutchV2 => {
            let uniswapx_strategy = UniswapXUniswapFill::new(
                Arc::new(provider.clone()),
                config.clone(),
                batch_sender,
                route_receiver,
            );
            engine.add_strategy(Box::new(uniswapx_strategy));
        }
        OrderType::Priority => {
            let priority_strategy = UniswapXPriorityFill::new(
                Arc::new(provider.clone()),
                config.clone(),
                batch_sender,
                route_receiver,
            );

            engine.add_strategy(Box::new(priority_strategy));
        }
    }

    let protect_executor = Box::new(ProtectExecutor::new(
        provider.clone(),
        mevblocker_provider.clone(),
    ));

    let public_tx_executor = Box::new(Public1559Executor::new(provider.clone(), provider.clone()));

    let protect_executor = ExecutorMap::new(protect_executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
        // No op for public transactions
        _ => None,
    });

    let public_tx_executor = ExecutorMap::new(public_tx_executor, |action| match action {
        Action::SubmitPublicTx(execution) => Some(execution),
        // No op for protected transactions
        _ => None,
    });

    engine.add_executor(Box::new(protect_executor));
    engine.add_executor(Box::new(public_tx_executor));
    // Start engine.
    match engine.run().await {
        Ok(mut set) => {
            while let Some(res) = set.join_next().await {
                match res {
                    Ok(res) => {
                        info!("res: {:?}", res);
                    }
                    Err(e) => {
                        info!("error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Engine run error: {:?}", e);
        }
    }
    Ok(())
}
