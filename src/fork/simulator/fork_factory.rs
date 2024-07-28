use std::sync::mpsc::channel as oneshot_channel;
use std::sync::Arc;

use ethers::{
    providers::{Provider, Ws},
    types::BlockId,
};
use revm::db::{CacheDB, EmptyDB};
use tracing_subscriber::registry::Data;

use super::{
    data_base_errors::DatabaseResult,
    fork_db::ForkDB,
    global_backend::{BackendFetchRequest, GlobalBackend},
};

use revm::primitives::{AccountInfo, Address as rAddress, U256 as rU256};

use futures::channel::mpsc::{channel, Sender};

#[derive(Clone)]
pub struct ForkFactory {
    backend: Sender<BackendFetchRequest>,
    initial_db: CacheDB<EmptyDB>,
}

impl ForkFactory {
    fn new(
        provider: Arc<Provider<Ws>>,
        initial_db: CacheDB<EmptyDB>,
        fork_block: Option<BlockId>,
    ) -> (Self, GlobalBackend) {
        let (backend, backend_rx) = channel(1);
        let handler = GlobalBackend::new(backend_rx, fork_block, provider, initial_db.clone());

        (
            Self {
                backend,
                initial_db,
            },
            handler,
        )
    }

    fn do_get_basic(&self, address: rAddress) -> DatabaseResult<Option<AccountInfo>> {
        tokio::task::block_in_place(|| {
            let (sender, rx) = oneshot_channel();
            let req = BackendFetchRequest::Basic(address, sender);
            self.backend.clone().try_send(req)?;
            rx.recv()?.map(Some)
        })
    }

    pub fn new_sandbox_factory(
        provider: Arc<Provider<Ws>>,
        initial_db: CacheDB<EmptyDB>,
        fork_block: Option<BlockId>,
    ) -> Self {
        let (shared, handler) = Self::new(provider, initial_db, fork_block);
        let _ = std::thread::Builder::new()
            .name("fork-backend-thread".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to create fork-backend-thread tokio runtime");

                rt.block_on(async move { handler.await });
            })
            .expect("failed to spawn backendhandler thread");

        shared
    }

    pub fn new_sandbox_fork(&self) -> ForkDB {
        ForkDB::new(self.backend.clone(), self.initial_db.clone())
    }

    pub fn insert_account_storage(
        &mut self,
        address: rAddress,
        slot: rU256,
        value: rU256,
    ) -> DatabaseResult<()> {
        if self.initial_db.accounts.get(&address).is_none() {
            // set basic info as its missing
            let info = match self.do_get_basic(address) {
                Ok(i) => i,
                Err(e) => return Err(e),
            };

            // keep record of fetched acc basic info
            if info.is_some() {
                self.initial_db.insert_account_info(address, info.unwrap());
            }
        }
        self.initial_db
            .insert_account_storage(address, slot, value)
            .unwrap();

        Ok(())
    }

    pub fn insert_account_info(&mut self, address: rAddress, info: AccountInfo) {
        self.initial_db.insert_account_info(address, info);
    }
}
