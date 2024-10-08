use alloy_sol_types::{sol, SolCall, SolValue};

use anyhow::Result;
use revm::primitives::Address as Add;
use revm::primitives::{Bytes, U256};

sol! {
    function getAmountOut(
        address pool,
        bool zeroForOne,
        uint256 amountIn
    ) external;
}

pub fn get_amount_out_calldata(pool: Add, token_in: Add, token_out: Add, amount_in: U256) -> Bytes {
    Bytes::from(
        getAmountOutCall {
            pool,
            zeroForOne: token_in < token_out,
            amountIn: amount_in,
        }
        .abi_encode(),
    )
}

pub fn decode_get_amount_out_response(response: Bytes) -> Result<u128> {
    let value = response.to_vec();
    let last_64_bytes = &value[value.len() - 64..];

    let (a, b) = match <(i128, i128)>::abi_decode(last_64_bytes, false) {
        Ok((a, b)) => (a, b),
        Err(e) => return Err(anyhow::anyhow!("'getAmountOut' decode failed: {:?}", e)),
    };
    let value_out = std::cmp::min(a, b);
    let value_out = -value_out;
    Ok(value_out as u128)
}
