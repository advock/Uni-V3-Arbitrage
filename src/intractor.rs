use alloy_sol_types::{sol, SolCall, SolValue};

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
