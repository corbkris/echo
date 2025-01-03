use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{Argument, PostgresError, PostgresQueryResult, DB},
    models::account::Account as ModelAccount,
};

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

    pub async fn insert(&self, account: &mut Account) -> Option<PostgresError> {
        self.db.insert(account).await
    }

    pub async fn update(&self, account: &mut Account) -> Option<PostgresError> {
        self.db.update(account).await
    }

    pub async fn delete(&self, account: &Account) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(account).await
    }

    pub async fn basic_search(
        &self,
        account: &Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Vec<Account>, PostgresError> {
        self.db.search_all(account, comparison, conditional).await
    }

    pub async fn basic_search_single(
        &self,
        account: &Account,
        comparison: StoreComparisonOperator,
        conditional: StoreConditionalOperator,
    ) -> Result<Account, PostgresError> {
        self.db.search(account, comparison, conditional).await
    }

    pub async fn get_by_id(&self) -> Result<Account, PostgresError> {
        let query = "";

        let mut args = vec![Argument::Int(5), Argument::Int(5)];
        args.push(Argument::Bool(true));

        self.db.query(query, args).await
    }
}
