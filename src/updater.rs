use crate::contract_modules::UniV3::types::{Pools, UniV3Data};
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

pub async fn start_updater(ws_provider: Arc<Provider<Ws>>, state: Arc<Mutex<State>>, from: U64) {
    let now = Instant::now();
    eprintln!("sfbhsfbhbs");

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
    eprintln!("sfbhsfbhbs");

    while from < U64::from(20074630) {
        eprintln!("block {:?}", from);
        update_block(ws_provider.clone(), state.clone(), from, sync_topic).await;
        eprint!("block {:?}", from);
        from += U64::one();
    }
}

pub async fn loop_block(
    ws_provider: Arc<Provider<Ws>>,
    state: Arc<Mutex<State>>,
    sync_topic: H256,
) {
    info!("block updater started ");
}

async fn update_block(
    ws_provider: Arc<Provider<Ws>>,
    state: Arc<Mutex<State>>,
    block: U64,
    sync_topic: H256,
) {
    let block: Block<TxHash> = match ws_provider.get_block(block).await {
        Ok(Some(d)) => d,
        Ok(None) => return,
        Err(error) => {
            println!("An error occurred: {}", error);
            return;
        }
    };

    let txes = block.transactions;

    for tx in txes {
        let tx_recipt = match ws_provider.get_transaction_receipt(tx).await {
            Ok(tx) => tx,
            Err(_) => continue,
        };

        if let Some(full_tx) = tx_recipt {
            let mut state_unlocked: tokio::sync::MutexGuard<State> = state.lock().await;

            let logs = full_tx.logs;
            for log in logs {
                let pointer = match state_unlocked.address_mapping.get(&log.address) {
                    Some(d) => *d,
                    None => continue,
                };

                for topic in &log.topics {
                    if *topic == sync_topic {
                        eprintln!("got pool");
                        let pool = Address::from_slice(&log.data[44..64]);
                        eprintln!("address of pool is {:?}", pool);

                        let x = Pools {
                            address: pool,
                            token0: Address::from_slice(&log.topics[1].as_bytes()[12..]),
                            token1: Address::from_slice(&log.topics[2].as_bytes()[12..]),
                        };

                        state_unlocked.pool_mapping.insert(pointer, x);
                    }
                }
            }
        }
    }
}
