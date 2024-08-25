use crate::caches::redis::account::AccountCache;
use echo_redis::generic::Cache;

#[derive(Clone)]
pub struct EchoCache {
    pub accounts: AccountCache,
}

impl EchoCache {
    pub fn new(cache: Cache) -> Self {
        Self {
            accounts: AccountCache::new(cache),
        }
    }
}
