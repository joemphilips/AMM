mod config;
mod api;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use bitcoin_rpc_provider::BitcoinCoreProvider;
use clap::Parser;
use dlc_manager::{self, Wallet};
use dlc_manager::{Oracle, SystemTimeProvider};
use dlc_sled_storage_provider;
use p2pd_oracle_client::P2PDOracleClient;
use tokio;

use crate::{api as dlc_amm_api, config::Opts};

pub(crate) type DlcManager = dlc_manager::manager::Manager<
    Arc<BitcoinCoreProvider>,
    Arc<BitcoinCoreProvider>,
    Box<dlc_sled_storage_provider::SledStorageProvider>,
    Box<P2PDOracleClient>,
    Arc<SystemTimeProvider>,
    Arc<BitcoinCoreProvider>,
>;

#[tokio::main]
async fn main() {
    let config = Opts::parse();
    let bitcoind_provider = Arc::new(
        bitcoin_rpc_provider::BitcoinCoreProvider::new(
            config.bitcoind_rpc_host,
            config.bitcoind_rpc_port,
            config.bitcoind_wallet,
            config.bitcoind_rpc_username,
            config.bitcoind_rpc_password,
        )
        .expect("Error creating BitcoinCoreProvider"),
    );
    let oracle_host = config.oracle_host;
    let oracle = tokio::task::spawn_blocking(move || {
        P2PDOracleClient::new(&oracle_host).expect("Error creating oracle client")
    })
    .await
    .unwrap();
    let mut oracles = HashMap::new();
    oracles.insert(oracle.get_public_key(), Box::new(oracle));
    let dlc_manager: Arc<Mutex<DlcManager>> = Arc::new(Mutex::new(
        DlcManager::new(
            bitcoind_provider.clone(),
            bitcoind_provider.clone(),
            Box::new(
                dlc_sled_storage_provider::SledStorageProvider::new(&config.datadir)
                    .expect("Error creating storage."),
            ),
            oracles,
            Arc::new(dlc_manager::SystemTimeProvider {}),
            bitcoind_provider.clone(),
        )
        .unwrap(),
    ));

    let runtime = tokio::runtime::Builder::new_multi_thread();
    println!("Hello, world!");
}
