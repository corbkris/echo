use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
};

pub struct EchoDatabase<'a> {
    pub accounts: &'a AccountStore<'a>,
    pub account_info: &'a AccountInfoStore<'a>,
    pub basic_account_info: &'a BasicAccountInfoStore<'a>,
    pub managed_account_info: &'a ManagedAccountInfoStore<'a>,
}

impl<'a> EchoDatabase<'a> {
    pub fn new(
        accounts: &'a AccountStore<'a>,
        account_info: &'a AccountInfoStore<'a>,
        basic_account_info: &'a BasicAccountInfoStore<'a>,
        managed_account_info: &'a ManagedAccountInfoStore<'a>,
    ) -> Self {
        Self {
            accounts,
            account_info,
            basic_account_info,
            managed_account_info,
        }
    }
}
