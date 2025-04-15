use echo_sql::{
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    table::BaseTable,
    tables::account_info::AccountInfo as TableAccountInfo,
};

pub type AccountInfo = TableAccountInfo;

impl_deref_store!(AccountInfoStore, AccountInfo);
pub struct AccountInfoStore<'a> {
    pub base_table: BaseTable<'a, AccountInfo>,
}

pub fn new_account_info_table<'a>(db: &'a DB) -> BaseTable<'a, AccountInfo> {
    BaseTable::<AccountInfo>::new(db)
}

impl<'a> AccountInfoStore<'a> {
    pub fn new(base_table: BaseTable<'a, AccountInfo>) -> Self {
        Self { base_table }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<AccountInfo, PostgresError> {
        let query = "
            SELECT ai.*
            FROM account_info ai
            INNER JOIN accounts a ON ai.account_id = a.id  
            WHERE a.username = $1 LIMIT 1;";

        self.query(query, vec![Argument::Str(username)]).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<AccountInfo, PostgresError> {
        let query = "
            SELECT ai.*
            FROM account_info ai
            INNER JOIN managed_account_info mai ON ai.id = mai.id
            WHERE mai.email = $1 LIMIT 1;";

        self.query(query, vec![Argument::Str(email)]).await
    }
}
