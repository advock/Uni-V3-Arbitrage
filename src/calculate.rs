use ethers::types::{Address, U256};
use revm::primitives::alloy_primitives::Uint;
use revm::primitives::alloy_primitives::I256;
use revm::primitives::alloy_primitives::U256 as U;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Deserialize)]
pub struct NetPositiveCycle {
    pub profit: I256,
    pub optimal_in: U256,
    pub swap_amounts: Vec<U256>,
    pub cycle_addresses: Vec<Address>,
}

impl Ord for NetPositiveCycle {
    fn cmp(&self, other: &Self) -> Ordering {
        other.profit.cmp(&self.profit)
    }
}

impl PartialOrd for NetPositiveCycle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NetPositiveCycle {}

// Ordering based on profit
impl PartialEq for NetPositiveCycle {
    fn eq(&self, other: &Self) -> bool {
        self.profit == other.profit
    }
}

pub fn maximize_profit(
    mut domain_min: U256,
    mut domain_max: U256,
    lowest_delta: U256,
    f: impl Fn(U) -> I256,
) -> U256 {
    loop {
        if domain_max > domain_min {
            if (domain_max - domain_min) > lowest_delta {
                let mid = (domain_min + domain_max) / 2;

                let lower_mid = (mid + domain_min) / 2;
                let upper_mid = (mid + domain_max) / 2;
                let lower_m = convert_u256_to_uint256(lower_mid);
                let upper_m = convert_u256_to_uint256(upper_mid);

                let f_output_lower = f(lower_m);
                let f_output_upper = f(upper_m);

                if f_output_lower > f_output_upper {
                    domain_max = mid;
                } else {
                    domain_min = mid;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (domain_max + domain_min) / 2
}

fn convert_u256_to_uint256(u256: U256) -> Uint<256, 4> {
    let mut bytes: [u8; 32] = [0u8; 32]; // U256 is 32 bytes
    u256.to_little_endian(&mut bytes); // fill bytes with U256 data
    Uint::<256, 4>::from_le_bytes(bytes)
}
