use crate::constants::WETH;
use crate::contract_modules::UniV3::types::Pools;
use crate::helper;
use crate::{contract_modules::UniV3::types::UniV3Pool, uniV3PoolGetter};
use abi::Hash;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use uniV3PoolGetter::Pool;

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
    pub pairs_mapping: HashMap<usize, RefCell<Pool>>,
    /// For easy access at pending state
    pub cycles_mapping: HashMap<Address, Vec<Cycle>>,
    // Mapping from usize to pool reserve
    pub pool_mapping: HashMap<usize, Pools>,
    // Real state of reserves to re apply after calc
    real_reserve_state: RefCell<HashMap<usize, [U256; 2]>>,
}

impl State {
    pub fn new_state(pairs: &[uniV3PoolGetter::Pool]) -> Self {
        let mut address_mapping = HashMap::new();
        let mut index_mapping = HashMap::new();
        let mut pairs_mapping = HashMap::new();

        for pair in pairs.iter() {
            let current_len = index_mapping.len();
            index_mapping.insert(current_len, pair.id);
            address_mapping.insert(pair.id, current_len);

            let token0 = address_mapping.contains_key(&pair.token0.id);

            if !token0 {
                let current_len = index_mapping.len();
                index_mapping.insert(current_len, pair.token0.id);
                address_mapping.insert(pair.token0.id, current_len);
            }

            let token1 = address_mapping.contains_key(&pair.token1.id);

            if !token1 {
                let current_len = index_mapping.len();
                index_mapping.insert(current_len, pair.token1.id);
                address_mapping.insert(pair.token1.id, current_len);
            }

            eprint!("in pairs iter");
        }
        eprint!("pairs done");
        let mut indexed_pairs = Vec::new();

        for pair in pairs {
            let indexted_pair = IndexedPair {
                address: *address_mapping.get(&pair.id).unwrap(),
                token0: *address_mapping.get(&pair.token0.id).unwrap(),
                token1: *address_mapping.get(&pair.token1.id).unwrap(),
            };

            indexed_pairs.push(indexted_pair);
            pairs_mapping.insert(
                *address_mapping.get(&pair.id).unwrap(),
                RefCell::new(pair.clone()),
            );

            eprint!("pairs 2");
        }

        eprint!("pairs 3");
        let weth_index = *address_mapping
            .get(&helper::convert_to_address(WETH))
            .unwrap();

        let cycles = Self::find_cycles(
            &indexed_pairs,
            weth_index,
            weth_index,
            4,
            &Vec::new(),
            &mut Vec::new(),
            &mut HashSet::new(),
        );
        eprint!("pairs 4");
        let mut cycles_mapping = HashMap::new();

        for indexed_cycle in cycles.iter() {
            for indexed_pair in indexed_cycle {
                cycles_mapping
                    .entry(index_mapping[&indexed_pair.address])
                    .or_insert_with(Vec::new)
                    .push(indexed_cycle.clone());
            }
        }

        let real_reserve_state = RefCell::new(HashMap::new());
        let pool_mapping: HashMap<usize, Pools> = HashMap::new();

        eprint!("wtf ");

        let state = Self {
            index_mapping,
            address_mapping,
            pairs_mapping,
            cycles_mapping,
            pool_mapping,
            real_reserve_state,
        };

        state
    }

    fn find_cycles(
        pairs: &[IndexedPair],
        token_in: usize,
        token_out: usize,
        max_hops: i32,
        current_pairs: &Vec<IndexedPair>,
        circles: &mut Vec<Cycle>,
        seen: &mut HashSet<usize>,
    ) -> Vec<Cycle> {
        let mut circles_copy = circles.clone();

        for pair in pairs {
            if seen.contains(&pair.address) {
                continue;
            }

            let temp_out: usize;

            if token_in == pair.token0 {
                temp_out = pair.token1;
            } else if token_in == pair.token1 {
                temp_out = pair.token0;
            } else {
                continue;
            }

            let mut new_seen = seen.clone();
            new_seen.insert(pair.address);

            if temp_out == token_out {
                let mut new_cycle = current_pairs.clone();
                new_cycle.push(*pair);
                circles_copy.push(new_cycle);
            } else if max_hops > 1 {
                let mut new_pairs: Vec<IndexedPair> = current_pairs.clone();
                new_pairs.push(*pair);
                circles_copy = Self::find_cycles(
                    pairs,
                    temp_out,
                    token_out,
                    max_hops - 1,
                    &new_pairs,
                    &mut circles_copy,
                    &mut new_seen,
                );
            }
        }
        circles_copy
    }
}
