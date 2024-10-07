pub mod config;
pub mod constants;
pub mod contract_modules;
pub mod data_collector;
pub mod filter;
pub mod fork;
pub mod helper;
pub mod intractor;
use ethers::prelude::*;
use eyre::Ok;
use log::info;
pub mod recon;
use std::cell::RefCell;
pub mod state;
use crate::contract_modules::UniV3::bindings::UniswapV3Router;
use crate::contract_modules::UniV3::types::UniV3Pool;
use crate::uniV3PoolGetter::PoolsData;
use anyhow::{anyhow, Result};
use constants::UniswapV3Pool;
use crossbeam_channel::unbounded;
use dotenv::dotenv;
use ethers::abi::Address;
use ethers::{
    contract::abigen,
    providers::{Middleware, Provider},
    types::H256,
};
use revm::primitives::alloy_primitives::Uint;
use revm::primitives::Address as Add;
use revm::primitives::Bytes;
use revm::primitives::{ExecutionResult, TransactTo};
use std::str::FromStr;
use tokio::sync::{Mutex, MutexGuard};
use tracing_subscriber::registry::Data;
pub mod states;
pub mod uniV3PoolGetter;
use crate::config::Config;
use std::sync::Arc;
pub mod updater;
pub mod utils;
use crate::constants::WETH;
use crate::state::State;
use alloy_provider::ProviderBuilder;
use contract_modules::UniV3::bindings;
use ethers::{types::U64, utils::keccak256};
use ethers_core::types::AddressOrBytes;
use helper::init_cache_db;
use helper::AlloyCacheDB;
use intractor::get_amount_out_calldata;
use revm::Evm;
use uniV3PoolGetter::{get_pools_list, Pool};

pub async fn run() {
    dotenv().ok();
    env_logger::init();

    get_pools_list("new_pools").await.unwrap();

    let storage = PoolsData::load_from_file("./new_pools").expect("failed loading data");
    eprint!("before getting stae");

    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(state::State::new_state(&storage.pools)));

    let config = Config::new().await;

    let http_url = std::env::var("NETWORK_HTTP").expect("missing NETWORK_RPC");
    let provider = ProviderBuilder::new().on_http(http_url.parse().unwrap());
    let provider = Arc::new(provider);

    let mut cache_db = init_cache_db(provider);

    // now that we have catche file.
    // now we need to find profitable cycles and then we will simulate that 0n cache dp
    // first to find profitable cycle we need to get the tx that's hitting mem pool
    // soooo

    let block_oracle = states::block_state::BlockOracle::new(config.wss.clone())
        .await
        .expect("Panic at block oracle creation");

    let (s, r) = crossbeam_channel::unbounded();
    recon::mempool::start_recon(
        state.clone(),
        config.wss_log.clone(),
        block_oracle.clone(),
        s,
    )
    .await;

    // what should this recon function do and how should it send txs here ?

    loop {
        let data = r.recv().unwrap();
        info!("tx received");
        let mut state: tokio::sync::MutexGuard<State> = state.lock().await;
        //let mut affected_pairs = Vec::new();
    }
}

// input all the potential cycles of affected pair:
pub fn cal_cycle_profit(state: &MutexGuard<State>, affected_pair: Option<Vec<Address>>) {
    let mut pointers: Vec<&Vec<crate::state::IndexedPair>> = Vec::new();

    match affected_pair {
        Some(affected_pair) => {
            affected_pair.iter().for_each(|pair_address| {
                if let Some(cycle) = state.cycles_mapping.get(pair_address) {
                    pointers.extend(cycle.iter());
                }
            });
        }
        None => {
            for (_, cycles) in &state.cycles_mapping {
                pointers.extend(cycles.iter());
            }
        }
    }

    //let mut net_profit_cycle = Vec::new();

    let weth = Address::from_str(WETH).unwrap();

    for cycle in pointers.clone() {
        let pairs = cycle
            .iter()
            .filter_map(|pair| state.pairs_mapping.get(&pair.address))
            .collect::<Vec<&RefCell<Pool>>>();

        let pairs_clone: Vec<&RefCell<Pool>> = pairs.clone();
        let profit_function =
            move |amount_in: U256| -> I256 { get_profit(weth, amount_in, &pairs_clone) };

        // here we need ti change get_profit
    }
}

pub fn get_profit(asset_in: Address, amount_in: U256, pairs: &Vec<&RefCell<Pool>>) -> I256 {
    let mut amount_out: U256 = amount_in;
    for pair in pairs {
        let pair = pair.borrow();
        let input = convert_u256_to_uint256(amount_in);

        let x = Add::from(pair.id.0);

        let calldata = get_amount_out_calldata(
            Add::from(pair.id.0),
            Add::from((Address::from_str(WETH)).unwrap().0),
            Add::from(pair.token1.id.0),
            input,
        );
    }

    U256::one();
}

pub fn volumes(from: U256, to: U256, count: usize) -> Vec<U256> {
    let start = U256::from(0);
    let mut volumes = Vec::new();
    let distance = to - from;
    let step = distance / U256::from(count);

    for i in 1..(count + 1) {
        let current = start + step * U256::from(i);
        volumes.push(current);
    }

    volumes.reverse();
    volumes
}

fn convert_u256_to_uint256(u256: U256) -> Uint<256, 4> {
    let mut bytes = [0u8; 32]; // U256 is 32 bytes
    u256.to_little_endian(&mut bytes); // fill bytes with U256 data
    Uint::<256, 4>::from_le_bytes(bytes)
}
