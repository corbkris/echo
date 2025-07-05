use echo_redis::connection::RedisClient;

use crate::caches::account::AccountCache;

pub struct EchoCache<'a> {
    pub accounts: AccountCache<'a>,
}

impl<'a> EchoCache<'a> {
    pub fn new(client: &'a RedisClient) -> Self {
        let accounts = AccountCache::new(client);
        Self { accounts }
    }
}
