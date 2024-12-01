use crate::business::accounts::service::Service as AccountService;

pub struct Wrapper<'a> {
    pub account_service: &'a AccountService<'a>,
}

impl<'a> Wrapper<'a> {
    pub fn new(account_service: &'a AccountService) -> Self {
        Self { account_service }
    }
}
