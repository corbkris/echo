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

    pub async fn find_by_username(&self, username: &str) -> Result<Account, PostgresError> {
        self.search(
            &Account {
                username: username.to_string(),
                ..Default::default()
            },
            ComparisonOperator::Equal,
            ConditonalOperator::Basic,
        )
        .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Account, PostgresError> {
        let query = "
            SELECT a.*
            FROM accounts a
            INNER JOIN account_info ai ON ai.account_id = a.id
            INNER JOIN managed_account_info mai ON mai.id = ai.id
            WHERE mai.email = $1 LIMIT 1;";

        self.query(query, vec![Argument::Str(email)]).await
    }

    pub async fn find_by_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Account, PostgresError> {
        let query = "
            SELECT a.*
            FROM accounts a
            INNER JOIN account_info ai ON a.id = ai.account_id 
            INNER JOIN managed_account_info mai ON ai.id = mai.id 
            WHERE mai.email = $1 AND ai.password = $2 LIMIT 1;";

        self.query(query, vec![Argument::Str(email), Argument::Str(password)])
            .await
    }

    pub async fn find_by_username_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Account, PostgresError> {
        let query = "
            SELECT a.*
            FROM accounts a
            INNER JOIN account_info ai ON a.id = ai.account_id 
            WHERE a.username = $1 AND ai.password = $2 LIMIT 1;";

        self.query(
            query,
            vec![Argument::Str(username), Argument::Str(password)],
        )
        .await
    }
}
