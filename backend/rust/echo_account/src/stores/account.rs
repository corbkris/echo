use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    table::BaseTable,
    tables::account::Account as ModelAccount,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type Account = ModelAccount;

impl_deref_store!(AccountStore, Account);
pub struct AccountStore<'a> {
    pub base_table: BaseTable<'a, Account>,
}

pub fn new_account_table<'a>(db: &'a DB) -> BaseTable<'a, Account> {
    BaseTable::<Account>::new(db)
}

impl<'a> AccountStore<'a> {
    pub fn new(base_table: BaseTable<'a, Account>) -> Self {
        Self { base_table }
    }

    pub async fn get_by_id(&self) -> Result<Account, PostgresError> {
        let query = "";

        let mut args = vec![Argument::Int(5), Argument::Int(5)];
        args.push(Argument::Bool(true));

        self.query(query, args).await
    }
}
