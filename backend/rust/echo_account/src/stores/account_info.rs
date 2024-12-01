use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account_info::AccountInfo as ModelAccountInfo,
};
use sqlx::{postgres::PgQueryResult, Error};

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

    pub async fn insert(&self, account_info: &mut AccountInfo) -> Option<Error> {
        self.db.insert(account_info).await
    }

    pub async fn update(&self, account_info: &mut AccountInfo) -> Option<Error> {
        self.db.update(account_info).await
    }

    pub async fn delete(&self, account_info: &AccountInfo) -> Result<PgQueryResult, Error> {
        self.db.delete(account_info).await
    }

    pub async fn basic_search(
        &self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<AccountInfo>, Error> {
        self.db
            .search_all(account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &self,
        account_info: &AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<AccountInfo, Error> {
        self.db.search(account_info, comparison, conditional).await
    }
}
