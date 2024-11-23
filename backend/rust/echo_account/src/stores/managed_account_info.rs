use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::managed_account_info::ManagedAccountInfo as ModelManagedAccountInfo,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type ManagedAccountInfo = ModelManagedAccountInfo;

#[derive(Clone)]
pub struct ManagedAccountInfoStore {
    db: DB,
}

impl ManagedAccountInfoStore {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn insert(
        &mut self,
        managed_account_info: ManagedAccountInfo,
    ) -> Result<ManagedAccountInfo, Error> {
        self.db.insert(managed_account_info).await
    }

    pub async fn update(
        &mut self,
        managed_account_info: ManagedAccountInfo,
    ) -> Result<ManagedAccountInfo, Error> {
        self.db.update(managed_account_info).await
    }

    pub async fn delete(
        &mut self,
        managed_account_info: ManagedAccountInfo,
    ) -> Result<PgQueryResult, Error> {
        self.db.delete(managed_account_info).await
    }

    pub async fn basic_search(
        &mut self,
        managed_account_info: ManagedAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<ManagedAccountInfo>, Error> {
        self.db
            .search_all(managed_account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &mut self,
        managed_account_info: ManagedAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<ManagedAccountInfo, Error> {
        self.db
            .search(managed_account_info, comparison, conditional)
            .await
    }
}
