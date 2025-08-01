use crate::business::wrapper::Services;
use crate::caches::wrapper::EchoCache;
use crate::logger::basic::register_subscriber;
use crate::queues::wrapper::EchoQue;
use crate::stores::wrapper::EchoDatabase;
use echo_rabbit::connection::{Config as RabbitConfig, RabbitConnection};
use echo_redis::connection::{Config as RedisConfig, RedisClient};
use echo_sql::connection::{Config as PostgresConfig, PostgresPool};
use tokio::sync::OnceCell;
use tracing;
use tracing_appender::non_blocking::WorkerGuard;

pub static ECHO_DEPENDENCY: OnceCell<Dependency> = OnceCell::const_new();
pub static ECHO_INFASTRUCTURE: OnceCell<Infastructure> = OnceCell::const_new();
pub static ECHO_SERVICE: OnceCell<Services> = OnceCell::const_new();

pub struct Dependency {
    pub gaurd: WorkerGuard,
    pub pool: PostgresPool,
    pub client: RedisClient,
    pub connection: RabbitConnection,
}

impl Dependency {
    pub async fn new() -> Self {
        let gaurd = register_subscriber("rustaccount.log");
        let pool = PostgresConfig::new().connect().await.unwrap();
        let client = RedisConfig::new().connect().unwrap();
        let connection = RabbitConfig::new().connect().await.unwrap();
        Self {
            gaurd,
            pool,
            client,
            connection,
        }
    }
}

pub struct Infastructure<'a> {
    pub echo_db: EchoDatabase<'a>,
    pub echo_cache: EchoCache<'a>,
    pub echo_que: EchoQue<'a>,
}

impl<'a> Infastructure<'a> {
    pub async fn new(dependency: &'a Dependency) -> Self {
        let echo_db = EchoDatabase::new(&dependency.pool);
        let echo_cache = EchoCache::new(&dependency.client);
        let echo_que = EchoQue::new(&dependency.connection).await;
        Self {
            echo_db,
            echo_cache,
            echo_que,
        }
    }
}

pub struct Common<'a> {
    pub gaurd: &'a WorkerGuard,
    pub db: &'a EchoDatabase<'a>,
    pub cache: &'a EchoCache<'a>,
    pub que: &'a EchoQue<'a>,
    pub services: &'a Services<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup_v2().await;
        tracing::info!("services started");
        Self {
            gaurd: &ECHO_DEPENDENCY.get().unwrap().gaurd,
            db: &ECHO_INFASTRUCTURE.get().unwrap().echo_db,
            cache: &ECHO_INFASTRUCTURE.get().unwrap().echo_cache,
            que: &ECHO_INFASTRUCTURE.get().unwrap().echo_que,
            services: ECHO_SERVICE.get().unwrap(),
        }
    }
}

async fn setup_v2() {
    ECHO_DEPENDENCY
        .get_or_init(|| async { Dependency::new().await })
        .await;

    ECHO_INFASTRUCTURE
        .get_or_init(|| async { Infastructure::new(ECHO_DEPENDENCY.get().unwrap()).await })
        .await;

    ECHO_SERVICE
        .get_or_init(|| async {
            Services::new(
                &ECHO_INFASTRUCTURE.get().unwrap().echo_db,
                &ECHO_INFASTRUCTURE.get().unwrap().echo_cache,
                &ECHO_INFASTRUCTURE.get().unwrap().echo_que,
            )
        })
        .await;
}
