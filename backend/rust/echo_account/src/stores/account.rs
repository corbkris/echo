use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    table::BaseTable,
    tables::account::Account as TableAccount,
};

pub type Account = TableAccount;

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

    pub async fn find_by_username(&self, username: String) -> Result<Account, PostgresError> {
        self.search(
            &Account {
                username,
                ..Default::default()
            },
            ComparisonOperator::Equal,
            ConditonalOperator::Basic,
        )
        .await
    }

    pub async fn find_by_id(&self) -> Result<Account, PostgresError> {
        let query = "";

        let mut args = vec![Argument::Int(5), Argument::Int(5)];
        args.push(Argument::Bool(true));

        self.query(query, args).await
    }
}
