use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::account::Account as ModelAccount,
};
use sqlx::{postgres::PgQueryResult, Error};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type Account = ModelAccount;

pub struct AccountStore<'a> {
    db: &'a DB<'a>,
}

impl<'a> AccountStore<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn insert(&self, account: &mut Account) -> Option<Error> {
        self.db.insert(account).await
    }

    pub async fn update(&self, account: &mut Account) -> Option<Error> {
        self.db.update(account).await
    }

    pub async fn delete(&self, account: &Account) -> Result<PgQueryResult, Error> {
        self.db.delete(account).await
    }

    pub async fn basic_search(
        &self,
        account: &Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<Account>, Error> {
        self.db.search_all(account, comparison, conditional).await
    }

    pub async fn basic_search_single(
        &self,
        account: &Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Account, Error> {
        self.db.search(account, comparison, conditional).await
    }
}
