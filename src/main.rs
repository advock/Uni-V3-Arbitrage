use ethers::{types::U64, utils::keccak256};
use futures::future::ok;
use EThDexMev::{config::Config, helper, updater};

#[tokio::main]
pub async fn main() {
    let mut config = Config::new().await;
    updater::start_updater(config.wss.clone(), U64::from(20074600));
}
