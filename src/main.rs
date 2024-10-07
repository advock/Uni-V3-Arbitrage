use abi::Address;
use crossbeam_channel::unbounded;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::{types::U64, utils::keccak256};
use ethers_core::types::AddressOrBytes;
use eyre::Ok;
use futures::future::ok;
use log::info;
use reqwest::Client;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use EThDexMev::constants::UniswapV3Pool;
use EThDexMev::contract_modules::UniV3::bindings::UniswapV3Router;
use EThDexMev::helper::bind;
use EThDexMev::state::State;
use EThDexMev::states;
use EThDexMev::uniV3PoolGetter::PoolsData;
use EThDexMev::{constants, recon};

use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use std::env;
use uniV3PoolGetter::get_pools_list;
use uniV3PoolGetter::AllPools;
use EThDexMev::run;
use EThDexMev::{config::Config, helper, uniV3PoolGetter, updater};

#[tokio::main]
async fn main() {
    run().await;
}
