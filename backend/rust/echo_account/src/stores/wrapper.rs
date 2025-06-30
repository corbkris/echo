use echo_sql::generic::DB;

use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
    signup_verification::SignupVerificationStore,
};

use super::{
    account::new_account_table, account_info::new_account_info_table,
    basic_account_info::new_basic_account_info_table,
    managed_account_info::new_managed_account_info_table,
    signup_verification::new_signup_verification_table,
};

pub struct EchoDatabase<'a> {
    pub accounts: Box<AccountStore<'a>>,
    pub account_info: Box<AccountInfoStore<'a>>,
    pub basic_account_info: Box<BasicAccountInfoStore<'a>>,
    pub managed_account_info: Box<ManagedAccountInfoStore<'a>>,
    pub signup_verification: Box<SignupVerificationStore<'a>>,
}

impl<'a> EchoDatabase<'a> {
    pub fn new(db: &'a DB) -> Self {
        let accounts = Box::new(AccountStore::new(new_account_table(db)));
        let account_info = Box::new(AccountInfoStore::new(new_account_info_table(db)));
        let basic_account_info =
            Box::new(BasicAccountInfoStore::new(new_basic_account_info_table(db)));
        let managed_account_info = Box::new(ManagedAccountInfoStore::new(
            new_managed_account_info_table(db),
        ));
        let signup_verification = Box::new(SignupVerificationStore::new(
            new_signup_verification_table(db),
        ));

        Self {
            accounts,
            account_info,
            basic_account_info,
            managed_account_info,
            signup_verification,
        }
    }
}
