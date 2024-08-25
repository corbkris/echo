use crate::stores::booth::BoothStore;
use echo_sql::generic::DB;

pub struct Database {
    pub booths: BoothStore,
}

impl Database {
    pub fn new(db: DB) -> Self {
        Self {
            booths: BoothStore::new(db),
        }
    }
}
