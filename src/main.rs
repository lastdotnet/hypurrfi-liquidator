use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::time_collector::TimeCollector;
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use executors::protect_executor::ProtectExecutor;
use std::sync::Arc;
use strategies::{
    aave_strategy::{AaveStrategy, Deployment},
    types::{Action, Config, Event},
};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

pub mod collectors;
pub mod executors;
pub mod strategies;

static POLL_INTERVAL_SECS: u64 = 60 * 5;

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub archive_rpc: String,

    #[arg(long)]
    pub write_rpc: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    /// Percentage of profit to pay in gas.
    #[arg(long)]
    pub bid_percentage: u64,

    #[arg(long)]
    pub deployment: Deployment,

    #[arg(long)]
    pub liquidator_address: String,

    #[arg(long)]
    pub chain_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("aave_v3_liquidator", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let archive_rpc = Http::from_str(&args.archive_rpc)?;
    let write_rpc = Http::from_str(&args.write_rpc)?;
    let archive_provider = Provider::new(archive_rpc);
    let write_provider = Provider::new(write_rpc);

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(args.chain_id);
    let address = wallet.address();

    let archive_provider = Arc::new(archive_provider.nonce_manager(address).with_signer(wallet.clone()));
    let write_provider = Arc::new(write_provider.nonce_manager(address).with_signer(wallet.clone()));

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up time collector.
    let time_collector = Box::new(TimeCollector::new(POLL_INTERVAL_SECS));
    let time_collector = CollectorMap::new(time_collector, Event::NewTick);
    engine.add_collector(Box::new(time_collector));

    let config = Config {
        bid_percentage: args.bid_percentage,
        chain_id: args.chain_id,
    };

    let strategy = AaveStrategy::new(
        Arc::new(archive_provider.clone()),
        Arc::new(write_provider.clone()),
        config,
        args.deployment,
        args.liquidator_address
    );
    engine.add_strategy(Box::new(strategy));

    let executor = Box::new(ProtectExecutor::new(write_provider.clone(), write_provider.clone()));

    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });

    engine.add_executor(Box::new(executor));
    info!("Starting engine");
    // Start engine.
    match engine.run().await {
        Ok(mut set) => {
            while let Some(res) = set.join_next().await {
                info!("res: {:?}", res);
            }
        }
        Err(e) => panic!("Error starting liquidator {}", e)
    }
    Ok(())
}
