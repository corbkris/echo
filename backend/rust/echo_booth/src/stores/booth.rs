use echo_sql::{
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    table::BaseTable,
    tables::booth::Booth as TableBooth,
};

pub type Booth = TableBooth;

impl_deref_store!(BoothStore, Booth);
pub struct BoothStore<'a> {
    pub base_table: BaseTable<'a, Booth>,
}

pub fn new_booth_table<'a>(db: &'a DB) -> BaseTable<'a, Booth> {
    BaseTable::<Booth>::new(db)
}

impl<'a> BoothStore<'a> {
    pub fn new(base_table: BaseTable<'a, Booth>) -> Self {
        Self { base_table }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Booth, PostgresError> {
        let query = "
            SELECT a.*
            FROM accounts a
            INNER JOIN account_info ai ON ai.account_id = a.id
            INNER JOIN managed_account_info mai ON mai.id = ai.id
            WHERE mai.email = $1 LIMIT 1;";

        self.query(query, vec![Argument::Str(email)]).await
    }
}
