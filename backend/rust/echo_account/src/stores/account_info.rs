use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    impl_deref_store,
    table::BaseTable,
    tables::account_info::AccountInfo as ModelAccountInfo,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type AccountInfo = ModelAccountInfo;

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
}
