use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::basic_account_info::BasicAccountInfo as ModelBasicAccountInfo,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type BasicAccountInfo = ModelBasicAccountInfo;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BasicAccountInfoStore {
    db: Arc<Mutex<DB>>,
}

impl BasicAccountInfoStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    pub async fn insert(
        &mut self,
        managed_account_info: BasicAccountInfo,
    ) -> Result<BasicAccountInfo, Error> {
        self.db.lock().await.insert(managed_account_info).await
    }

    pub async fn update(
        &mut self,
        managed_account_info: BasicAccountInfo,
    ) -> Result<BasicAccountInfo, Error> {
        self.db.lock().await.update(managed_account_info).await
    }

    pub async fn delete(
        &mut self,
        managed_account_info: BasicAccountInfo,
    ) -> Result<PgQueryResult, Error> {
        self.db.lock().await.delete(managed_account_info).await
    }

    pub async fn basic_search(
        &mut self,
        managed_account_info: BasicAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<BasicAccountInfo>, Error> {
        self.db
            .lock()
            .await
            .search_all(managed_account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &mut self,
        managed_account_info: BasicAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<BasicAccountInfo, Error> {
        self.db
            .lock()
            .await
            .search(managed_account_info, comparison, conditional)
            .await
    }
}
