use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{PostgresError, PostgresQueryResult, DB},
    models::basic_account_info::BasicAccountInfo as ModelBasicAccountInfo,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type BasicAccountInfo = ModelBasicAccountInfo;

pub struct BasicAccountInfoStore<'a> {
    db: &'a DB<'a>,
}

impl<'a> BasicAccountInfoStore<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn insert(
        &self,
        managed_account_info: &mut BasicAccountInfo,
    ) -> Option<PostgresError> {
        self.db.insert(managed_account_info).await
    }

    pub async fn update(
        &self,
        managed_account_info: &mut BasicAccountInfo,
    ) -> Option<PostgresError> {
        self.db.update(managed_account_info).await
    }

    pub async fn delete(
        &self,
        managed_account_info: &BasicAccountInfo,
    ) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(managed_account_info).await
    }

    pub async fn basic_search(
        &self,
        managed_account_info: &BasicAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<BasicAccountInfo>, PostgresError> {
        self.db
            .search_all(managed_account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &self,
        managed_account_info: &BasicAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<BasicAccountInfo, PostgresError> {
        self.db
            .search(managed_account_info, comparison, conditional)
            .await
    }
}
