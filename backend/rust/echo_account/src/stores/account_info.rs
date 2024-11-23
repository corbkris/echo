use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account_info::AccountInfo as ModelAccountInfo,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type AccountInfo = ModelAccountInfo;

#[derive(Clone)]
pub struct AccountInfoStore {
    db: DB,
}

impl AccountInfoStore {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn insert(&mut self, account_info: AccountInfo) -> Result<AccountInfo, Error> {
        self.db.insert(account_info).await
    }

    pub async fn update(&mut self, account_info: AccountInfo) -> Result<AccountInfo, Error> {
        self.db.update(account_info).await
    }

    pub async fn delete(&mut self, account_info: AccountInfo) -> Result<PgQueryResult, Error> {
        self.db.delete(account_info).await
    }

    pub async fn basic_search(
        &mut self,
        account_info: AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<AccountInfo>, Error> {
        self.db
            .search_all(account_info, comparison, conditional)
            .await
    }

    pub async fn basic_search_single(
        &mut self,
        account_info: AccountInfo,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<AccountInfo, Error> {
        self.db.search(account_info, comparison, conditional).await
    }
}
