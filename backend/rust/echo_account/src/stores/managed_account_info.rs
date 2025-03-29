use echo_sql::{
    generic::DB, impl_deref_store, table::BaseTable,
    tables::managed_account_info::ManagedAccountInfo as TableManagedAccountInfo,
};

pub type ManagedAccountInfo = TableManagedAccountInfo;

impl_deref_store!(ManagedAccountInfoStore, ManagedAccountInfo);
pub struct ManagedAccountInfoStore<'a> {
    pub base_table: BaseTable<'a, ManagedAccountInfo>,
}

pub fn new_managed_account_info_table<'a>(db: &'a DB) -> BaseTable<'a, ManagedAccountInfo> {
    BaseTable::<ManagedAccountInfo>::new(db)
}

impl<'a> ManagedAccountInfoStore<'a> {
    pub fn new(base_table: BaseTable<'a, ManagedAccountInfo>) -> Self {
        Self { base_table }
    }
}
