use alloy_provider::{network::Ethereum, Provider, RootProvider};
use alloy_transport_http::Http;
use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers::types::Address as add;
use reqwest::Client;
use revm::db::{AlloyDB, CacheDB};
use revm::primitives::{Address, Bytes, ExecutionResult, Output, TransactTo, U256};
use revm::Evm;
use std::sync::Arc;

pub type AlloyCacheDB = CacheDB<AlloyDB<Http<Client>, Ethereum, Arc<RootProvider<Http<Client>>>>>;

pub fn convert_to_address(address: &str) -> add {
    address.parse::<add>().unwrap()
}

pub fn bind(name: &str, abi: &str) {
    let name: String = name.to_string();
    let bindings = Abigen::new(&name, abi).unwrap().generate().unwrap();
    let path: String = format!("src/contract_modules/UniV3/bindings/{}.rs", name);
    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(_) => {}
    }
    bindings.write_to_file(&path).unwrap();
}

pub fn init_cache_db(provider: Arc<RootProvider<Http<Client>>>) -> AlloyCacheDB {
    CacheDB::new(AlloyDB::new(provider, Default::default()))
}

pub fn revm_revert(
    from: Address,
    to: Address,
    calldata: Bytes,
    cache_db: &mut AlloyCacheDB,
) -> Result<Bytes> {
    let mut evm = Evm::builder()
        .with_db(cache_db)
        .modify_tx_env(|tx| {
            tx.caller = from;
            tx.transact_to = TransactTo::Call(to);
            tx.data = calldata;
            tx.value = U256::from(0);
        })
        .build();
    let ref_tx = evm.transact().unwrap();
    let result = ref_tx.result;

    let value = match result {
        ExecutionResult::Revert { output: value, .. } => value,
        _ => {
            panic!("It should never happen!");
        }
    };

    Ok(value)
}

pub fn revm_call(
    from: Address,
    to: Address,
    calldata: Bytes,
    cache_db: &mut AlloyCacheDB,
) -> Result<Bytes> {
    let mut evm = Evm::builder()
        .with_db(cache_db)
        .modify_tx_env(|tx| {
            tx.caller = from;
            tx.transact_to = TransactTo::Call(to);
            tx.data = calldata;
            tx.value = U256::from(0);
        })
        .build();

    let ref_tx = evm.transact().unwrap();
    let result = ref_tx.result;

    let value = match result {
        ExecutionResult::Success {
            output: Output::Call(value),
            ..
        } => value,
        result => {
            return Err(anyhow!("execution failed: {result:?}"));
        }
    };

    Ok(value)
}
