use crate::business::wrapper::Wrapper;
use crate::caches::wrapper::EchoCache;
use crate::logger::basic::register_subscriber;
use crate::queues::wrapper::EchoQue;
use crate::stores::wrapper::EchoDatabase;
use echo_rabbit::{
    connection::{Config as RabbitConfig, RabbitConnection},
    generic::Que,
};
use echo_redis::{
    connection::{Config as RedisConfig, RedisClient},
    generic::Cache,
};
use echo_sql::{
    connection::{Config as PostgresConfig, PostgresPool},
    generic::DB,
};
use tokio::sync::OnceCell;
use tracing;
use tracing_appender::non_blocking::WorkerGuard;

pub static ECHO_DEPENDENCY: OnceCell<Dependency> = OnceCell::const_new();
pub static ECHO_BUILDER: OnceCell<Buidler> = OnceCell::const_new();
pub static ECHO_INFASTRUCTURE: OnceCell<Infastructure> = OnceCell::const_new();
pub static ECHO_SERVICE: OnceCell<Wrapper> = OnceCell::const_new();

pub struct Dependency {
    pub gaurd: Box<WorkerGuard>,
    pub pool: Box<PostgresPool>,
    pub client: Box<RedisClient>,
    pub connection: Box<RabbitConnection>,
}

impl Dependency {
    pub async fn new() -> Self {
        let gaurd = Box::new(register_subscriber("rustaccount.log"));
        let pool = Box::new(PostgresConfig::new().connect().await.unwrap());
        let client = Box::new(RedisConfig::new().connect().unwrap());
        let connection = Box::new(RabbitConfig::new().connect().await.unwrap());
        Self {
            gaurd,
            pool,
            client,
            connection,
        }
    }
}

pub struct Buidler<'a> {
    pub db: Box<DB<'a>>,
    pub cache: Box<Cache<'a>>,
    pub que: Box<Que<'a>>,
}

impl<'a> Buidler<'a> {
    pub fn new(dependencies: &'a Dependency) -> Self {
        let db = Box::new(DB::new(&dependencies.pool));
        let cache = Box::new(Cache::new(&dependencies.client));
        let que = Box::new(Que::new(&dependencies.connection));
        Self { db, cache, que }
    }
}

pub struct Infastructure<'a> {
    pub echo_db: Box<EchoDatabase<'a>>,
    pub echo_cache: Box<EchoCache<'a>>,
    pub echo_que: Box<EchoQue<'a>>,
}

impl<'a> Infastructure<'a> {
    pub async fn new(builder: &'a Buidler<'a>) -> Self {
        let echo_db = Box::new(EchoDatabase::new(&builder.db));
        let echo_cache = Box::new(EchoCache::new(&builder.cache));
        let echo_que = Box::new(EchoQue::new(&builder.que).await);
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
    pub services: &'a Wrapper<'a>,
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

    ECHO_BUILDER
        .get_or_init(|| async { Buidler::new(ECHO_DEPENDENCY.get().unwrap()) })
        .await;

    ECHO_INFASTRUCTURE
        .get_or_init(|| async { Infastructure::new(ECHO_BUILDER.get().unwrap()).await })
        .await;

    ECHO_SERVICE
        .get_or_init(|| async {
            Wrapper::new(
                &ECHO_INFASTRUCTURE.get().unwrap().echo_db,
                &ECHO_INFASTRUCTURE.get().unwrap().echo_cache,
                &ECHO_INFASTRUCTURE.get().unwrap().echo_que,
            )
        })
        .await;
}
