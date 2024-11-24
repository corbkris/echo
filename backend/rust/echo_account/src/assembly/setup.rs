use crate::business::wrapper::Wrapper;
use crate::{
    caches::redis::wrapper::EchoCache, queues::wrapper::EchoQue, stores::wrapper::EchoDatabase,
};
use echo_rabbit::{
    connection::{BasicConnection, Config as RabbitConfig},
    generic::Que,
};
use echo_redis::{
    connection::{BasicClient, Config as RedisConfig},
    generic::Cache,
};
use echo_sql::{connection::Config as PgConfig, generic::DB};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Common {
    pub db: EchoDatabase,
    pub cache: EchoCache,
    pub que: EchoQue,
    pub services: Wrapper,
}

impl Common {
    pub async fn new() -> Result<Self, String> {
        let postgres = match PgConfig::new().connect().await {
            Ok(postgres) => postgres,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let db_store = DB::new(postgres);
        let db = EchoDatabase::new(Arc::new(Mutex::new(db_store)));

        let config = RedisConfig::new();
        let basic_client = match BasicClient::new(config) {
            Ok(basic_client) => basic_client,
            Err(err) => return Err(err.to_string()),
        };
        let redis_cache = Cache::new(basic_client.client);
        let cache = EchoCache::new(redis_cache);

        let rabbit_config = RabbitConfig::new();
        let basic_connection = match BasicConnection::new(rabbit_config).await {
            Ok(basic_connection) => basic_connection,
            Err(err) => return Err(err.to_string()),
        };
        let rabbit_que = Que::new(basic_connection.connection);
        let que = EchoQue::new(rabbit_que).await;

        let services = Wrapper::new(db.clone(), cache.clone(), que.clone());

        Ok(Self {
            db,
            cache,
            que,
            services,
        })
    }
}
