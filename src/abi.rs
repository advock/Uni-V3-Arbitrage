use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::Result;
use ethers::types::U256;
use revm::primitives::alloy_primitives::Signed;
use revm::primitives::alloy_primitives::Uint;
use revm::primitives::alloy_primitives::U256 as U;
use revm::primitives::{Address, Bytes, I256};
sol! {
    struct PoolSequence{
        address pool1;
        address pool2;
        address pool3;
    }

    function getCycleReturn(
        PoolSequence calldata pools,
        uint256 amountIn,
        bool firstSwapZeroForOne
    ) external returns (int256 finalAmountOut);
}

pub fn decode_get_cycle_return_response(response: Bytes) -> Result<I256> {
    let (value,) = <(I256,)>::abi_decode(&response, false)?;
    Ok(value)
}

pub fn get_cycle_calldata(sequence: PoolSequence, amount_in: U) -> Bytes {
    Bytes::from(
        getCycleReturnCall {
            pools: sequence,
            amountIn: amount_in,
            firstSwapZeroForOne: false,
        }
        .abi_encode(),
    )
}

pub fn convert_u256_to_uint256(u256: U256) -> Uint<256, 4> {
    let mut bytes: [u8; 32] = [0u8; 32]; // U256 is 32 bytes
    u256.to_little_endian(&mut bytes); // fill bytes with U256 data
    Uint::<256, 4>::from_le_bytes(bytes)
}
