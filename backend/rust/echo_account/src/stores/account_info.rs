use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{PostgresError, PostgresQueryResult, DB},
    models::account_info::AccountInfo as ModelAccountInfo,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type AccountInfo = ModelAccountInfo;

pub struct AccountInfoStore<'a> {
    db: &'a DB<'a>,
}

impl<'a> AccountInfoStore<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn insert(&self, account_info: &mut AccountInfo) -> Option<PostgresError> {
        self.db.insert(account_info).await
    }

    pub async fn update(&self, account_info: &mut AccountInfo) -> Option<PostgresError> {
        self.db.update(account_info).await
    }

    pub async fn delete(
        &self,
        account_info: &AccountInfo,
    ) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(account_info).await
    }

    pub async fn basic_search(
        &self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<AccountInfo>, PostgresError> {
        self.db
            .search_all(account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<AccountInfo, PostgresError> {
        self.db.search(account_info, comparison, conditional).await
    }
}
