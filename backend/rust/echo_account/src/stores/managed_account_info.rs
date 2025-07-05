use echo_sql::{
    connection::PostgresPool, generic::DB, impl_deref_store, table::BaseTable,
    tables::managed_account_info::ManagedAccountInfo as TableManagedAccountInfo,
};

pub type ManagedAccountInfo = TableManagedAccountInfo;

impl_deref_store!(ManagedAccountInfoStore, ManagedAccountInfo);
pub struct ManagedAccountInfoStore<'a> {
    pub base_table: BaseTable<'a, ManagedAccountInfo>,
}

pub fn new_managed_account_info_table<'a>(
    pool: &'a PostgresPool,
) -> BaseTable<'a, ManagedAccountInfo> {
    BaseTable::<ManagedAccountInfo>::new(DB::new(pool))
}

impl<'a> ManagedAccountInfoStore<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        Self {
            base_table: new_managed_account_info_table(pool),
        }
    }
}
