use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
};
use echo_sql::generic::DB;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct EchoDatabase {
    pub accounts: AccountStore,
    pub account_info: AccountInfoStore,
    pub basic_account_info: BasicAccountInfoStore,
    pub managed_account_info: ManagedAccountInfoStore,
}

impl EchoDatabase {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self {
            accounts: AccountStore::new(db.clone()),
            account_info: AccountInfoStore::new(db.clone()),
            basic_account_info: BasicAccountInfoStore::new(db.clone()),
            managed_account_info: ManagedAccountInfoStore::new(db.clone()),
        }
    }
}
