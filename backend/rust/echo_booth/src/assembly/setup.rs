use crate::stores::wrapper::Database;
use echo_sql::{connection::Config, generic::DB};

pub struct Common {
    pub db: Database,
}

impl Common {
    pub async fn new() -> Self {
        let postgres = Config::new().connect().await.unwrap();
        let db_store = DB::new(postgres);
        let db = Database::new(db_store);
        Self { db }
    }
}
