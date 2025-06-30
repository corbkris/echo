use echo_redis::generic::Cache;

use crate::caches::account::AccountCache;

pub struct EchoCache<'a> {
    pub accounts: Box<AccountCache<'a>>,
}

impl<'a> EchoCache<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        let accounts = Box::new(AccountCache::new(cache));
        Self { accounts }
    }
}
