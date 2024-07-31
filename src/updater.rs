use crate::contract_modules::{self, UniV3::types::UniV3Pool};
use ethers::abi::{AbiDecode, Token};
use ethers::prelude::*;
use hex;
use log::*;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use crate::constants::SYNC;
use crate::state::State;

pub async fn start_updater(ws_provider: Arc<Provider<Ws>>, from: U64) {
    let now = Instant::now();

    let decode = hex::decode(SYNC).unwrap();
    let sync_topic = H256::from_slice(&decode);

    let mut from = from;

    let block = match ws_provider.get_block_number().await {
        Ok(d) => d,
        Err(err) => {
            error!("An error occurred: {}", err);
            return;
        }
    };

    while from < block {
        info!("block {:?}", block);
        update_block(ws_provider.clone(), block, sync_topic);
    }
}

pub async fn loop_block(
    ws_provider: Arc<Provider<Ws>>,
    state: Arc<Mutex<State>>,
    sync_topic: H256,
) {
    info!("block updater started ");
}

async fn update_block(ws_provider: Arc<Provider<Ws>>, block: U64, sync_topic: H256) {
    let block: Block<TxHash> = match ws_provider.get_block(block).await {
        Ok(Some(d)) => d,
        Ok(None) => return,
        Err(error) => {
            println!("An error occurred: {}", error);
            return;
        }
    };

    let mut pairs = vec![];

    let txes = block.transactions;

    for tx in txes {
        let tx_recipt = match ws_provider.get_transaction_receipt(tx).await {
            Ok(tx) => tx,
            Err(_) => continue,
        };

        if let Some(full_tx) = tx_recipt {
            let logs = full_tx.logs;
            for log in logs {
                for topic in log.topics {
                    if topic == sync_topic {
                        let pool = Address::from_slice(&log.data[32..]);
                        info!("address of pool is {:?}", pool);
                        pairs.push(pool);

                        // let (token0, token1, pool): (Address, Address, Address) =
                        //     match AbiDecode::decode(&log.data) {
                        //         Ok(decoded) => decoded,
                        //         Err(_) => continue,
                        //     };

                        // let fee: U256 = U256::from_big_endian(&topic[3].as_bytes());
                        // let (tick_spacing, pool): (i32, Address) =
                        //     match AbiDecode::decode(&log.data) {
                        //         Ok(decoded) => decoded,
                        //         Err(_) => continue,
                        //     };
                    }
                }
            }
        }
    }
}
