use ethers::prelude::*;

pub const EXECUATOR_ADDRESS: &str = "0x....";
pub const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

pub const UNISWAP_V2: [(&str, &str, &str, u32); 1] = [(
    "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D", // V2Router02
    "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f", // Factory Contract
    "0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f",
    9970,
)];

abigen!(UniSwapV3, "src/abi/UniV3Factory.json");
