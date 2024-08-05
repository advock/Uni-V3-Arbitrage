use std::ops::Add;

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniV3Pool {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    pub reserve0: U256,
    pub reserve1: U256,
    pub router_fee: U256,
    pub fees: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pools {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniV3Data {
    pub list: Vec<Pools>,
}

impl UniV3Data {
    pub fn new(list: Vec<Pools>) -> Self {
        Self { list }
    }
    pub fn save_to_file(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        let serialized = serde_json::to_string_pretty(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
