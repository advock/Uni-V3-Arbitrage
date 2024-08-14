use ethers::providers::{Provider, Ws};
use std::sync::Arc;

use crate::uniV3PoolGetter;

pub async fn update_reserve(pairs: &mut [uniV3PoolGetter::Pool], wss_provider: Arc<Provider<Ws>>) {}
