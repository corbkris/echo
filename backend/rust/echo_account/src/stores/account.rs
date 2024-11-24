use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account::Account as ModelAccount,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type Account = ModelAccount;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AccountStore {
    db: Arc<Mutex<DB>>,
}

impl AccountStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    pub async fn insert(&mut self, account: Account) -> Result<Account, Error> {
        self.db.lock().await.insert(account).await
    }

    pub async fn update(&mut self, account: Account) -> Result<Account, Error> {
        self.db.lock().await.update(account).await
    }

    pub async fn delete(&mut self, account: Account) -> Result<PgQueryResult, Error> {
        self.db.lock().await.delete(account).await
    }

    pub async fn basic_search(
        &mut self,
        account: Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<Account>, Error> {
        self.db
            .lock()
            .await
            .search_all(account, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &mut self,
        account: Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Account, Error> {
        self.db
            .lock()
            .await
            .search(account, comparison, conditional)
            .await
    }
}
