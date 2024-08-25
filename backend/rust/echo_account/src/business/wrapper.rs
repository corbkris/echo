use crate::business::accounts::service::Service as account_service;
use crate::{
    caches::redis::wrapper::EchoCache, queues::wrapper::EchoQue, stores::wrapper::EchoDatabase,
};

pub struct Wrapper {
    pub account_service: account_service,
}

impl Wrapper {
    pub fn new(db: EchoDatabase, cache: EchoCache, que: EchoQue) -> Self {
        Self {
            account_service: account_service::new(db, cache, que),
        }
    }
}
