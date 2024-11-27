use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account_info::AccountInfo as ModelAccountInfo,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type AccountInfo = ModelAccountInfo;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AccountInfoStore {
    db: Arc<Mutex<DB>>,
}

impl AccountInfoStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    pub async fn insert(&mut self, account_info: &AccountInfo) -> Result<AccountInfo, Error> {
        self.db.lock().await.insert(account_info).await
    }

    pub async fn update(&mut self, account_info: &AccountInfo) -> Result<AccountInfo, Error> {
        self.db.lock().await.update(account_info).await
    }

    pub async fn delete(&mut self, account_info: &AccountInfo) -> Result<PgQueryResult, Error> {
        self.db.lock().await.delete(account_info).await
    }

    pub async fn basic_search(
        &mut self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<AccountInfo>, Error> {
        self.db
            .lock()
            .await
            .search_all(account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &mut self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<AccountInfo, Error> {
        self.db
            .lock()
            .await
            .search(account_info, comparison, conditional)
            .await
    }
}
