use crate::{caches::wrapper::EchoCache, queues::wrapper::EchoQue, stores::wrapper::EchoDatabase};

use super::accounts::service::Service;

pub struct Wrapper<'a> {
    pub account_service: Box<Service<'a>>,
}

impl<'a> Wrapper<'a> {
    pub fn new(db: &'a EchoDatabase, cache: &'a EchoCache, que: &'a EchoQue) -> Self {
        let account_service = Box::new(Service::new(db, cache, que));
        Self { account_service }
    }
}
