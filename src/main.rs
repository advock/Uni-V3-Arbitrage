use crossbeam_channel::unbounded;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::{types::U64, utils::keccak256};
use eyre::Ok;
use futures::future::ok;
use log::info;
use reqwest::Client;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use EThDexMev::recon;
use EThDexMev::state::State;
use EThDexMev::uniV3PoolGetter::PoolsData;

use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use std::env;
use uniV3PoolGetter::get_pools_list;
use uniV3PoolGetter::AllPools;
use EThDexMev::{config::Config, helper, uniV3PoolGetter, updater};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let file_name = "pools_output";

    let config = Config::new().await;

    let block = config.wss.get_block_number().await.unwrap() - 10000;

    // get_pools_list(file_name).await.unwrap();
    eprint!("before getting storage");
    let storage = PoolsData::load_from_file("./pools_output").expect("failed loading data");
    eprint!("before getting stae");
    let state = Arc::new(Mutex::new(State::new_state(&storage.pools)));
    updater::start_updater(Arc::clone(&config.wss), state.clone(), block);

    let (s, r) = crossbeam_channel::unbounded();

    recon::mempool::start_recon(state, config.wss, block_oracle, s).await;
}
