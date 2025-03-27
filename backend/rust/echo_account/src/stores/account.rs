use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    models::account::Account as ModelAccount,
    table::BaseTable,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type Account = ModelAccount;

impl_deref_store!(AccountStore, Account);
pub struct AccountStore<'a> {
    db: &'a DB<'a>,
    pub base_table: BaseTable<'a, Account>,
}

pub fn new_account_table<'a>(db: &'a DB) -> BaseTable<'a, Account> {
    BaseTable::<Account>::new(db)
}

impl<'a> AccountStore<'a> {
    pub fn new(db: &'a DB, base_table: BaseTable<'a, Account>) -> Self {
        Self { db, base_table }
    }

    pub async fn get_by_id(&self) -> Result<Account, PostgresError> {
        let query = "";

        let mut args = vec![Argument::Int(5), Argument::Int(5)];
        args.push(Argument::Bool(true));

        self.db.query(query, args).await
    }
}
