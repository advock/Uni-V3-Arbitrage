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
use helper::revm_call;
use log::info;
pub mod recon;
use std::cell::RefCell;
pub mod calculate;
use std::rc::Rc;
pub mod state;
use crate::contract_modules::UniV3::bindings::UniswapV3Router;
use crate::contract_modules::UniV3::types::UniV3Pool;
use crate::uniV3PoolGetter::PoolsData;
use calculate::maximize_profit;
use calculate::NetPositiveCycle;
use constants::UniswapV3Pool;
use constants::EXECUATOR_ADDRESS;
use crossbeam_channel::unbounded;
use dotenv::dotenv;
use ethers::abi::Address;
use ethers::types::Bytes as ethBytes;
use ethers::types::U256 as u652;
use ethers::{
    contract::abigen,
    providers::{Middleware, Provider},
    types::H256,
};
use eyre::{Report, Result};
use intractor::decode_get_amount_out_response;
use revm::primitives::alloy_primitives::{Uint, I256, U256};

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
use helper::revm_revert;
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

    let cache_db = Rc::new(RefCell::new(init_cache_db(provider)));

    // now that we have catche file.
    // now we need to find profitable cycles and then we will simulate that 0n cache dp
    // first to find profitable cycle we need to get the tx that's hitting mem pool
    // soooo

    let decoded = hex::decode(constants::SWAP).unwrap();
    let swap_topic = H256::from_slice(&decoded);

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
    let mut cache_db_ref = cache_db.borrow_mut();
    loop {
        let main_data: recon::mempool::FutureTx = r.recv().unwrap();
        info!("tx received");
        let mut state: tokio::sync::MutexGuard<State> = state.lock().await;

        let mut affected_pairs = Vec::new();

        for log in main_data.logs {
            let topics = match log.topics {
                Some(d) => d,
                None => continue,
            };
            let data = match log.data {
                Some(d) => d,
                None => continue,
            };

            let address = match log.address {
                Some(d) => d,
                None => continue,
            };

            for tpoic in topics {
                if tpoic == swap_topic {
                    affected_pairs.push(address);
                    // here we need to call this function on revm
                    revm_call(
                        Add::from(main_data.tx.from.0),
                        Add::from(main_data.tx.to.unwrap().0),
                        ethers_to_revm(main_data.tx.input.clone()),
                        &mut cache_db_ref,
                    )
                    .unwrap();
                }
            }
        }

        let mut potential_cycles =
            cal_cycle_profit(&state, Some(affected_pairs.clone()), cache_db.clone());
    }
}

// input all the potential cycles of affected pair:
pub fn cal_cycle_profit(
    state: &MutexGuard<State>,
    affected_pair: Option<Vec<Address>>,
    cache_db: Rc<RefCell<AlloyCacheDB>>,
) -> Vec<NetPositiveCycle> {
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
    let mut net_profit_cycles = Vec::new();

    let weth = Address::from_str(WETH).unwrap();

    for cycle in pointers.clone() {
        let pairs = cycle
            .iter()
            .filter_map(|pair| state.pairs_mapping.get(&pair.address))
            .collect::<Vec<&RefCell<Pool>>>();

        let pairs_clone: Vec<&RefCell<Pool>> = pairs.clone();
        let cache_db_clone = cache_db.clone();
        let profit_function = move |amount_in: U256| -> I256 {
            get_profit(weth, amount_in, &pairs_clone, cache_db_clone.clone()).unwrap()
        };

        let optimal: u652 = maximize_profit(
            u652::one(),
            u652::from_dec_str("10000000000000000000000").unwrap(),
            u652::from_dec_str("10").unwrap(),
            profit_function,
        );
        // this needs to be changed
        let (profit, swap_address) = get_profit_of_cycle(optimal, weth);

        let mut cycle_internal = Vec::new();
        for pair in pairs {
            cycle_internal.push(pair.borrow().id);
        }

        if profit > I256::ONE {
            let net_positive_cycle = NetPositiveCycle {
                profit,
                optimal_in: optimal,
                swap_amounts: swap_address,
                cycle_addresses: cycle_internal,
            };
            net_profit_cycles.push(net_positive_cycle);
        }
    }
    net_profit_cycles.sort();
    net_profit_cycles.into_iter().take(5).collect()
}

pub fn get_profit_of_cycle(amount_in: u652, token_in: Address) -> (I256, Vec<u652>) {
    unimplemented!()
}

pub fn get_profit(
    asset_in: Address,
    amount_in: U256,
    pairs: &Vec<&RefCell<Pool>>,
    cache_db: Rc<RefCell<AlloyCacheDB>>,
) -> Result<I256> {
    // let http_url = std::env::var("NETWORK_HTTP").expect("missing NETWORK_RPC");
    // let provider = ProviderBuilder::new().on_http(http_url.parse().unwrap());
    // let provider = Arc::new(provider);
    //let mut cache_db = init_cache_db(provider);
    // Use eyre::Result for error handling

    let mut cache_db = cache_db.borrow_mut();
    let mut amount_out: U256 = amount_in;
    let mut token_in: Address = asset_in;
    for pair in pairs {
        let pair = pair.borrow();
        let input = amount_out;
        let token_out: Address;
        if token_in == pair.token0.id {
            token_out = pair.token1.id
        } else {
            token_out = pair.token0.id
        }

        let calldata = get_amount_out_calldata(
            Add::from(pair.id.0),
            Add::from(token_in.0),
            Add::from(token_out.0),
            input,
        );

        if token_in == pair.token0.id {
            token_in = pair.token1.id
        } else {
            token_in = pair.token0.id
        }

        // Return errors as eyre::Result<_, Report>
        let response: Bytes = revm_revert(
            Add::from_str(EXECUATOR_ADDRESS).unwrap(),
            Add::from(pair.id.0),
            calldata,
            &mut cache_db,
        )
        .unwrap();

        amount_out = U256::try_from(decode_get_amount_out_response(response).unwrap()).unwrap();

        print!("{:?} amount out {:?} -------", pair.token1.id, amount_out);
    }

    Ok(I256::from_raw(amount_out) - I256::from_raw(amount_in)) // Return I256 as Ok
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

fn ethers_to_revm(ethers_bytes: ethBytes) -> Bytes {
    Bytes(ethers_bytes.0) // Access the inner Vec<u8> and construct revm::Bytes
}
