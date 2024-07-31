use crate::{
    constants::{UniV3Factory, UniswapV3Pool},
    contract_modules::UniV3::types,
};

use axum::middleware;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UniV3Poo {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    pub reserve0: U256,
    pub reserve1: U256,
    pub router_fee: U256,
    pub fees0: U256,
    pub fees1: U256,
}

// pub async fn get_batch_pairs<M: Middleware>(
//     factory: H160,
//     from: U256,
//     step: U256,
//     middleware: Arc<M>,
// ) -> Vec<UniV3Poo> {
//     let mut pairs = vec![];
// }
