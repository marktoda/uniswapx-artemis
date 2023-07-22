use anyhow::Result;
use clap::Parser;

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::{
    block_collector::BlockCollector,
    uniswapx_order_collector::{UniswapXOrderCollector, CHAIN_ID},
    uniswapx_route_collector::UniswapXRouteCollector,
};
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Http, Provider, Ws},
    signers::{LocalWallet, Signer},
};
use executors::protect_executor::ProtectExecutor;
use std::sync::Arc;
use strategies::{
    types::{Action, Config, Event},
    uniswapx_strategy::UniswapXUniswapFill,
};
use tokio::sync::mpsc::channel;
use tracing::{info, Level};
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

    let mevblocker_provider =
        Provider::<Http>::try_from(MEV_BLOCKER).expect("could not instantiate HTTP Provider");

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(CHAIN_ID);
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));
    let mevblocker_provider = Arc::new(
        mevblocker_provider
            .nonce_manager(address)
            .with_signer(wallet),
    );

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up block collector.
    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    let (batch_sender, batch_receiver) = channel(512);
    let (route_sender, route_receiver) = channel(512);

    let uniswapx_collector = Box::new(UniswapXOrderCollector::new());
    let uniswapx_collector =
        CollectorMap::new(uniswapx_collector, |e| Event::UniswapXOrder(Box::new(e)));
    engine.add_collector(Box::new(uniswapx_collector));

    let uniswapx_route_collector =
        Box::new(UniswapXRouteCollector::new(batch_receiver, route_sender));
    let uniswapx_route_collector = CollectorMap::new(uniswapx_route_collector, |e| {
        Event::UniswapXRoute(Box::new(e))
    });
    engine.add_collector(Box::new(uniswapx_route_collector));

    let config = Config {
        bid_percentage: args.bid_percentage,
    };

    let strategy = UniswapXUniswapFill::new(
        Arc::new(provider.clone()),
        config,
        batch_sender,
        route_receiver,
    );
    engine.add_strategy(Box::new(strategy));

    let executor = Box::new(ProtectExecutor::new(
        provider.clone(),
        mevblocker_provider.clone(),
    ));

    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });

    engine.add_executor(Box::new(executor));
    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }
    Ok(())
}
