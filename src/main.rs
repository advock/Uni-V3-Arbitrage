use dotenv::dotenv;
use ethers::{types::U64, utils::keccak256};
use eyre::Ok;
use futures::future::ok;
use log::info;
use reqwest::Client;
use std::error::Error;

use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use std::env;
use uniV3PoolGetter::get_pools_list;
use uniV3PoolGetter::AllPools;
use EThDexMev::{config::Config, helper, uniV3PoolGetter, updater};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let file_name = "pools_output";
    get_pools_list(file_name).await?;
    Ok(())
}
