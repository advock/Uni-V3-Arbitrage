use alloy_sol_types::{sol, SolCall, SolValue};

use crate::calculate::convert_u256_to_uint256;
use crate::constants::EXECUATOR_ADDRESS;
use anyhow::Result;
use ethers::abi::Address;
use ethers::types::U256 as u652;
use ethers_core::k256::elliptic_curve::consts::U2;
use revm::primitives::Address as Add;
use revm::primitives::{uint, Bytes, U256};

sol! {
    struct PoolSequence {
        address pool1;
        address pool2;
        address pool3;
    }

    function executeCycleSwap(
        PoolSequence calldata pools,
        uint256 amountIn,
        bool firstSwapZeroForOne,
        address recipient
    ) external;
}

pub fn get_execute_cycle_swap_data(pools: Vec<Address>, amount: u652, recipient: Add) -> Bytes {
    let param = PoolSequence {
        pool1: Add::from(pools[0].0),
        pool2: Add::from(pools[1].0),
        pool3: Add::from(pools[2].0),
    };

    Bytes::from(
        executeCycleSwapCall {
            pools: param,
            amountIn: convert_u256_to_uint256(amount),
            firstSwapZeroForOne: false,
            recipient: recipient,
        }
        .abi_encode(),
    )
}
