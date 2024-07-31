use crate::contract_modules::UniV3::types::UniV3Pool;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IndexedPair {
    pub address: usize,

    pub token0: usize,
    pub token1: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cycles {
    cycles: Vec<Vec<IndexedPair>>,
    block: U256,
}

pub type Cycle = Vec<IndexedPair>;

pub struct State {
    /// For indexed pointer to address
    pub index_mapping: HashMap<usize, Address>,
    /// For address to indexed pointer
    pub address_mapping: HashMap<Address, usize>,
    /// Pointer to the pool
    pub pairs_mapping: HashMap<usize, RefCell<UniV3Pool>>,
    /// For easy access at pending state
    pub cycles_mapping: HashMap<Address, Vec<Cycle>>,
    // Real state of reserves to re apply after calc
    real_reserve_state: RefCell<HashMap<usize, [U256; 2]>>,
}
