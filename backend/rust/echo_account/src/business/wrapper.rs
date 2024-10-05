use crate::business::accounts::service::Service as account_service;
use crate::{
    caches::redis::wrapper::EchoCache, queues::wrapper::EchoQue, stores::wrapper::EchoDatabase,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Wrapper {
    pub account_service: Arc<Mutex<account_service>>,
}

impl Wrapper {
    pub fn new(db: EchoDatabase, cache: EchoCache, que: EchoQue) -> Self {
        Self {
            account_service: Arc::new(Mutex::new(account_service::new(db, cache, que))),
        }
    }
}
