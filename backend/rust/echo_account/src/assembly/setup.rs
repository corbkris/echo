use echo_sql::{
    connection::{Config as PostgresConfig, PostgresPool},
    generic::DB,
};
use tokio::sync::OnceCell;

use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
    wrapper::EchoDatabase,
};
use crate::{
    caches::{account::AccountCache, wrapper::EchoCache},
    stores::account::new_account_table,
};
use echo_redis::{
    connection::{Config as RedisConfig, RedisClient},
    generic::Cache,
};

use echo_rabbit::{
    connection::{Config as RabbitConfig, RabbitConnection},
    generic::{Que, RabbitChannel},
};

use crate::queues::{email::EmailQue, wrapper::EchoQue};

use crate::business::{accounts::service::Service as AccountService, wrapper::Wrapper};

pub static ECHO_DB: OnceCell<EchoDatabase> = OnceCell::const_new();
pub static ECHO_POOL: OnceCell<PostgresPool> = OnceCell::const_new();
pub static ECHO_POSTGRES: OnceCell<DB> = OnceCell::const_new();
pub static ECHO_ACCOUNT_STORE: OnceCell<AccountStore> = OnceCell::const_new();
pub static ECHO_ACCOUNT_INFO_STORE: OnceCell<AccountInfoStore> = OnceCell::const_new();
pub static ECHO_BASIC_ACCOUNT_INFO_STORE: OnceCell<BasicAccountInfoStore> = OnceCell::const_new();
pub static ECHO_MANAGED_ACCOUNT_STORE: OnceCell<ManagedAccountInfoStore> = OnceCell::const_new();

pub static ECHO_CACHE: OnceCell<EchoCache> = OnceCell::const_new();
pub static ECHO_CLIENT: OnceCell<RedisClient> = OnceCell::const_new();
pub static ECHO_REDIS: OnceCell<Cache> = OnceCell::const_new();
pub static ECHO_ACCOUNT_CACHE: OnceCell<AccountCache> = OnceCell::const_new();

pub static ECHO_QUEUES: OnceCell<EchoQue> = OnceCell::const_new();
pub static QUE_CONNECTION: OnceCell<RabbitConnection> = OnceCell::const_new();
pub static QUE_QUEUE: OnceCell<Que> = OnceCell::const_new();
pub static QUE_EMAIL_QUEUE: OnceCell<EmailQue> = OnceCell::const_new();
pub static QUE_EMAIL_CHANNEL: OnceCell<RabbitChannel> = OnceCell::const_new();

pub static ECHO_SERVICES: OnceCell<Wrapper> = OnceCell::const_new();
pub static ECHO_ACCOUNT_SERVICES: OnceCell<AccountService> = OnceCell::const_new();

pub struct Common<'a> {
    pub db: &'a EchoDatabase<'a>,
    pub cache: &'a EchoCache<'a>,
    pub que: &'a EchoQue<'a>,
    pub services: &'a Wrapper<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup().await;
        let db = ECHO_DB.get().unwrap();
        let cache = ECHO_CACHE.get().unwrap();
        let que = ECHO_QUEUES.get().unwrap();
        let services = ECHO_SERVICES.get().unwrap();
        Self {
            db,
            cache,
            que,
            services,
        }
    }
}

async fn get_pool() -> PostgresPool {
    PostgresConfig::new().connect().await.unwrap()
}

async fn set_pool() {
    ECHO_POOL.get_or_init(get_pool).await;
}

async fn get_postgres<'a>() -> DB<'a> {
    DB::new(ECHO_POOL.get().unwrap())
}

async fn set_postgres() {
    ECHO_POSTGRES.get_or_init(get_postgres).await;
}

async fn get_account_store<'a>() -> AccountStore<'a> {
    AccountStore::new(
        ECHO_POSTGRES.get().unwrap(),
        new_account_table(ECHO_POSTGRES.get().unwrap()),
    )
}

async fn set_account_store() {
    ECHO_ACCOUNT_STORE.get_or_init(get_account_store).await;
}

async fn get_account_info_store<'a>() -> AccountInfoStore<'a> {
    AccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_account_info_store() {
    ECHO_ACCOUNT_INFO_STORE
        .get_or_init(get_account_info_store)
        .await;
}

async fn get_basic_account_info_store<'a>() -> BasicAccountInfoStore<'a> {
    BasicAccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_basic_account_info_store() {
    ECHO_BASIC_ACCOUNT_INFO_STORE
        .get_or_init(get_basic_account_info_store)
        .await;
}

async fn get_managed_account_info_store<'a>() -> ManagedAccountInfoStore<'a> {
    ManagedAccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_managed_account_info_store() {
    ECHO_MANAGED_ACCOUNT_STORE
        .get_or_init(get_managed_account_info_store)
        .await;
}

async fn get_db<'a>() -> EchoDatabase<'a> {
    EchoDatabase::new(
        ECHO_ACCOUNT_STORE.get().unwrap(),
        ECHO_ACCOUNT_INFO_STORE.get().unwrap(),
        ECHO_BASIC_ACCOUNT_INFO_STORE.get().unwrap(),
        ECHO_MANAGED_ACCOUNT_STORE.get().unwrap(),
    )
}

async fn set_db() {
    ECHO_DB.get_or_init(get_db).await;
}

async fn get_client() -> RedisClient {
    RedisConfig::new().connect().unwrap()
}

async fn set_client() {
    ECHO_CLIENT.get_or_init(get_client).await;
}

async fn get_redis<'a>() -> Cache<'a> {
    Cache::new(ECHO_CLIENT.get().unwrap())
}

async fn set_redis() {
    ECHO_REDIS.get_or_init(get_redis).await;
}

async fn get_account_cache<'a>() -> AccountCache<'a> {
    AccountCache::new(ECHO_REDIS.get().unwrap())
}

async fn set_account_cache() {
    ECHO_ACCOUNT_CACHE.get_or_init(get_account_cache).await;
}

async fn get_echo_cache<'a>() -> EchoCache<'a> {
    EchoCache::new(ECHO_ACCOUNT_CACHE.get().unwrap())
}

async fn set_echo_cache() {
    ECHO_CACHE.get_or_init(get_echo_cache).await;
}

async fn get_connection() -> RabbitConnection {
    RabbitConfig::new().connect().await.unwrap()
}

async fn set_connection() {
    QUE_CONNECTION.get_or_init(get_connection).await;
}

async fn get_que<'a>() -> Que<'a> {
    Que::new(QUE_CONNECTION.get().unwrap())
}

async fn set_que() {
    QUE_QUEUE.get_or_init(get_que).await;
}

async fn get_email_que<'a>() -> EmailQue<'a> {
    EmailQue::new(QUE_QUEUE.get().unwrap())
}

async fn set_email_que() {
    QUE_EMAIL_QUEUE.get_or_init(get_email_que).await;
}

async fn get_email_channel<'a>() -> RabbitChannel {
    QUE_EMAIL_QUEUE
        .get()
        .unwrap()
        .create_email_channel()
        .await
        .unwrap()
}

async fn set_email_channel() {
    QUE_EMAIL_CHANNEL.get_or_init(get_email_channel).await;
}

async fn get_echo_que<'a>() -> EchoQue<'a> {
    EchoQue::new(
        QUE_EMAIL_QUEUE.get().unwrap(),
        QUE_EMAIL_CHANNEL.get().unwrap(),
    )
}

async fn set_echo_que() {
    ECHO_QUEUES.get_or_init(get_echo_que).await;
}

async fn get_account_service<'a>() -> AccountService<'a> {
    AccountService::new(
        ECHO_DB.get().unwrap(),
        ECHO_CACHE.get().unwrap(),
        ECHO_QUEUES.get().unwrap(),
    )
}

async fn set_account_service() {
    ECHO_ACCOUNT_SERVICES.get_or_init(get_account_service).await;
}

async fn get_services<'a>() -> Wrapper<'a> {
    Wrapper::new(ECHO_ACCOUNT_SERVICES.get().unwrap())
}

async fn set_services() {
    ECHO_SERVICES.get_or_init(get_services).await;
}

pub async fn setup() {
    set_pool().await;
    set_postgres().await;
    set_account_store().await;
    set_account_info_store().await;
    set_basic_account_info_store().await;
    set_managed_account_info_store().await;
    set_db().await;

    set_client().await;
    set_redis().await;
    set_account_cache().await;
    set_echo_cache().await;

    set_connection().await;
    set_que().await;
    set_email_que().await;
    set_email_channel().await;
    set_echo_que().await;

    set_account_service().await;
    set_services().await;
}
