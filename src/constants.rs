use ethers::prelude::*;

pub const EXECUATOR_ADDRESS: &str = "0x....";
pub const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
// below event is pool created
pub const SYNC: &str = "783cca1c0412dd0d695e784568c96da2e9c22ff989357a2e8b1d9b2b4e6b7118";
pub const SYNC_SWAP: &str = "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1";

pub const SWAP: &str = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";

pub const UNI_V3_Router: &str = "0xE592427A0AEce92De3Edee1F18E0157C05861564";

pub const UNISWAP_V3: [(&str, &str, &str, u32); 1] = [(
    "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D", // V2Router02
    "0x1F98431c8aD98523631AE4a59f267346ea31F984", // Factory Contract
    "0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f",
    9970,
)];

abigen!(UniV3Factory, "src/abi/UniV3Factory.json");
abigen!(UniswapV3Pool, "src/abi/UniswapV3Pool.json");
abigen!(UniswapV3Router, "src/abi/UniswapV3Router.json");
