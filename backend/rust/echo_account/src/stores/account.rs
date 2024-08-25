use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account::Account as ModelAccount,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type Account = ModelAccount;

#[derive(Clone)]
pub struct AccountStore {
    db: DB,
}

impl AccountStore {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn insert(&mut self, account: Account) -> Result<Account, Error> {
        self.db.insert(account).await
    }

    pub async fn update(&mut self, account: Account) -> Result<Account, Error> {
        self.db.update(account).await
    }

    pub async fn delete(&mut self, account: Account) -> Result<PgQueryResult, Error> {
        self.db.delete(account).await
    }

    pub async fn basic_search(
        &mut self,
        account: Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<Account>, Error> {
        self.db.search_all(account, comparison, conditional).await
    }

    pub async fn basic_search_single(
        &mut self,
        account: Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Account, Error> {
        self.db.search(account, comparison, conditional).await
    }
}
