use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{PostgresError, PostgresQueryResult, DB},
    models::managed_account_info::ManagedAccountInfo as ModelManagedAccountInfo,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type ManagedAccountInfo = ModelManagedAccountInfo;

pub struct ManagedAccountInfoStore<'a> {
    db: &'a DB<'a>,
}

impl<'a> ManagedAccountInfoStore<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn insert(
        &self,
        managed_account_info: &mut ManagedAccountInfo,
    ) -> Option<PostgresError> {
        self.db.insert(managed_account_info).await
    }

    pub async fn update(
        &self,
        managed_account_info: &mut ManagedAccountInfo,
    ) -> Option<PostgresError> {
        self.db.update(managed_account_info).await
    }

    pub async fn delete(
        &self,
        managed_account_info: &ManagedAccountInfo,
    ) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(managed_account_info).await
    }

    pub async fn basic_search(
        &self,
        managed_account_info: &ManagedAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<ManagedAccountInfo>, PostgresError> {
        self.db
            .search_all(managed_account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &self,
        managed_account_info: &ManagedAccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<ManagedAccountInfo, PostgresError> {
        self.db
            .search(managed_account_info, comparison, conditional)
            .await
    }
}
