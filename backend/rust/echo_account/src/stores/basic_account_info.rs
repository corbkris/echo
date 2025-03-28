use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    impl_deref_store,
    table::BaseTable,
    tables::basic_account_info::BasicAccountInfo as ModelBasicAccountInfo,
};

pub type StoreConditionalOperator = ConditonalOperator;
pub type StoreComparisonOperator = ComparisonOperator;
pub type BasicAccountInfo = ModelBasicAccountInfo;

impl_deref_store!(BasicAccountInfoStore, BasicAccountInfo);
pub struct BasicAccountInfoStore<'a> {
    pub base_table: BaseTable<'a, BasicAccountInfo>,
}

pub fn new_basic_account_info_table<'a>(db: &'a DB) -> BaseTable<'a, BasicAccountInfo> {
    BaseTable::<BasicAccountInfo>::new(db)
}

impl<'a> BasicAccountInfoStore<'a> {
    pub fn new(base_table: BaseTable<'a, BasicAccountInfo>) -> Self {
        Self { base_table }
    }
}
