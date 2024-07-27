use std::sync::mpsc::channel;

use futures::channel::mpsc::Sender;

use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    primitives::{
        Account, AccountInfo, Address as rAddress, Bytecode as rBytecode, HashMap as rHashMap,
        B256, KECCAK_EMPTY, U256 as rU256,
    },
    Database, DatabaseCommit,
};

use super::data_base_errors::{DatabaseError, DatabaseResult};
use crate::fork::simulator::global_backend::BackendFetchRequest;

pub struct ForkDB {
    backend: Sender<BackendFetchRequest>,
    db: CacheDB<EmptyDB>,
}

impl ForkDB {
    pub fn new(backend: Sender<BackendFetchRequest>, db: CacheDB<EmptyDB>) -> Self {
        Self { backend, db }
    }
}
