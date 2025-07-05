use crate::{caches::wrapper::EchoCache, queues::wrapper::EchoQue, stores::wrapper::EchoDatabase};

use super::accounts::service::Service as AccountService;

pub struct Services<'a> {
    pub account_service: AccountService<'a>,
}

impl<'a> Services<'a> {
    pub fn new(db: &'a EchoDatabase, cache: &'a EchoCache, que: &'a EchoQue) -> Self {
        let account_service = AccountService::new(db, cache, que);
        Self { account_service }
    }
}
