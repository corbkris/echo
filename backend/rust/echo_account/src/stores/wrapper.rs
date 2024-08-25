use crate::stores::account::AccountStore;
use echo_sql::generic::DB;

#[derive(Clone)]
pub struct EchoDatabase {
    pub accounts: AccountStore,
}

impl EchoDatabase {
    pub fn new(db: DB) -> Self {
        Self {
            accounts: AccountStore::new(db),
        }
    }
}
