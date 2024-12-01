use crate::caches::account::AccountCache;

pub struct EchoCache<'a> {
    pub accounts: &'a AccountCache<'a>,
}

impl<'a> EchoCache<'a> {
    pub fn new(accounts: &'a AccountCache) -> Self {
        Self { accounts }
    }
}
