use echo_sql::connection::PostgresPool;

use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
    signup_verification::SignupVerificationStore,
};

pub struct EchoDatabase<'a> {
    pub accounts: AccountStore<'a>,
    pub account_info: AccountInfoStore<'a>,
    pub basic_account_info: BasicAccountInfoStore<'a>,
    pub managed_account_info: ManagedAccountInfoStore<'a>,
    pub signup_verification: SignupVerificationStore<'a>,
}

impl<'a> EchoDatabase<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        let accounts = AccountStore::new(pool);
        let account_info = AccountInfoStore::new(pool);
        let basic_account_info = BasicAccountInfoStore::new(pool);
        let managed_account_info = ManagedAccountInfoStore::new(pool);
        let signup_verification = SignupVerificationStore::new(pool);

        Self {
            accounts,
            account_info,
            basic_account_info,
            managed_account_info,
            signup_verification,
        }
    }
}
