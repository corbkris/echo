use echo_sql::{
    connection::PostgresPool, generic::DB, impl_deref_store, table::BaseTable,
    tables::basic_account_info::BasicAccountInfo as TableBasicAccountInfo,
};

pub type BasicAccountInfo = TableBasicAccountInfo;

impl_deref_store!(BasicAccountInfoStore, BasicAccountInfo);
pub struct BasicAccountInfoStore<'a> {
    pub base_table: BaseTable<'a, BasicAccountInfo>,
}

pub fn new_basic_account_info_table<'a>(pool: &'a PostgresPool) -> BaseTable<'a, BasicAccountInfo> {
    BaseTable::<BasicAccountInfo>::new(DB::new(pool))
}

impl<'a> BasicAccountInfoStore<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        Self {
            base_table: new_basic_account_info_table(pool),
        }
    }
}
