use anyhow::Result;
use clap::{ArgGroup, Parser};

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::uniswapx_order_collector::OrderType;
use collectors::{
    block_collector::BlockCollector, uniswapx_order_collector::UniswapXOrderCollector,
    uniswapx_route_collector::UniswapXRouteCollector,
};
use ethers::{
    providers::{Http, Provider, Ws},
    signers::{LocalWallet, Signer},
};
use executors::protect_executor::ProtectExecutor;
use executors::public_1559_executor::Public1559Executor;
use std::collections::HashMap;
use std::sync::Arc;
use strategies::keystore::KeyStore;
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
#[command(group(
    ArgGroup::new("Key_source")
        .required(true)
        .args(&["private_key", "aws_secret_arn"])
))]
#[command(group(
    ArgGroup::new("key_source")
        .args(&["private_key", "private_key_file", "aws_secret_arn"])
))]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,

    /// Private key for sending txs.
    #[arg(long, group = "key_source")]
    pub private_key: Option<String>,

    /// Path to file containing mapping between public address and private key.
    #[arg(long, group = "key_source")]
    pub private_key_file: Option<String>,

    /// AWS secret arn for fetching private key.
    /// This is a secret manager arn that contains the private key as plain text.
    #[arg(long, group = "key_source")]
    pub aws_secret_arn: Option<String>,

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

    let key_store = Arc::new(KeyStore::new());

    if let Some(aws_secret_arn) = args.aws_secret_arn {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_secretsmanager::Client::new(&config);
        let pk_mapping_json = client
            .get_secret_value()
            .secret_id(aws_secret_arn)
            .send()
            .await
            .expect("could not get private key secret")
            .secret_string
            .expect("secret string not found");
        let pk_mapping = serde_json::from_str::<HashMap<String, String>>(&pk_mapping_json)
            .expect("could not parse private key mapping");
        // load into keystore
        for (address, pk) in pk_mapping {
            key_store.add_key(address, pk).await;
        }
    } else if let Some(pk_file) = args.private_key_file {
        let pk_mapping_json = std::fs::read_to_string(pk_file).expect("could not read pk file");
        let pk_mapping = serde_json::from_str::<HashMap<String, String>>(&pk_mapping_json)
            .expect("could not parse private key mapping");
        // load into keystore
        for (address, pk) in pk_mapping {
            key_store.add_key(address, pk).await;
        }
    } else {
        let pk = args.private_key.clone().unwrap();
        let wallet: LocalWallet = pk.parse::<LocalWallet>().unwrap().with_chain_id(chain_id);
        let address = wallet.address();
        key_store.add_key(address.to_string(), pk).await;
    }
    info!("Key store initialized with {} keys", key_store.len().await);

    let provider = Arc::new(provider);
    let mevblocker_provider = Arc::new(mevblocker_provider);

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
        key_store.clone(),
    ));

    let public_tx_executor = Box::new(Public1559Executor::new(
        provider.clone(),
        provider.clone(),
        key_store.clone(), // TODO: this should be the same as the protect executor
    ));

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
