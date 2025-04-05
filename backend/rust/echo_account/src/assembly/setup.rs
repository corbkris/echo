use crate::business::{accounts::service::Service as AccountService, wrapper::Wrapper};
use crate::queues::{email::EmailQue, wrapper::EchoQue};
use crate::{
    caches::{account::AccountCache, wrapper::EchoCache},
    stores::account::new_account_table,
};
use crate::{
    logger::basic::register_subscriber,
    stores::{
        account::AccountStore,
        account_info::{new_account_info_table, AccountInfoStore},
        basic_account_info::{new_basic_account_info_table, BasicAccountInfoStore},
        managed_account_info::{new_managed_account_info_table, ManagedAccountInfoStore},
        signup_verification::{new_signup_verification_table, SignupVerificationStore},
        wrapper::EchoDatabase,
    },
};
use echo_rabbit::{
    connection::{Config as RabbitConfig, RabbitConnection},
    generic::{Que, RabbitChannel},
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
use tracing::info;

pub static ECHO_POOL: OnceCell<PostgresPool> = OnceCell::const_new();
pub static ECHO_POSTGRES: OnceCell<DB> = OnceCell::const_new();
pub static ECHO_ACCOUNT_STORE: OnceCell<AccountStore> = OnceCell::const_new();
pub static ECHO_ACCOUNT_INFO_STORE: OnceCell<AccountInfoStore> = OnceCell::const_new();
pub static ECHO_BASIC_ACCOUNT_INFO_STORE: OnceCell<BasicAccountInfoStore> = OnceCell::const_new();
pub static ECHO_MANAGED_ACCOUNT_INFO_STORE: OnceCell<ManagedAccountInfoStore> =
    OnceCell::const_new();
pub static ECHO_SIGNUP_VERIFICATION_STORE: OnceCell<SignupVerificationStore> =
    OnceCell::const_new();
pub static ECHO_DB: OnceCell<EchoDatabase> = OnceCell::const_new();

pub static ECHO_CLIENT: OnceCell<RedisClient> = OnceCell::const_new();
pub static ECHO_REDIS: OnceCell<Cache> = OnceCell::const_new();
pub static ECHO_ACCOUNT_CACHE: OnceCell<AccountCache> = OnceCell::const_new();
pub static ECHO_CACHE: OnceCell<EchoCache> = OnceCell::const_new();

pub static ECHO_CONNECTION: OnceCell<RabbitConnection> = OnceCell::const_new();
pub static ECHO_QUEUE: OnceCell<Que> = OnceCell::const_new();
pub static ECHO_EMAIL_QUEUE: OnceCell<EmailQue> = OnceCell::const_new();
pub static ECHO_EMAIL_CHANNEL: OnceCell<RabbitChannel> = OnceCell::const_new();
pub static ECHO_QUEUES: OnceCell<EchoQue> = OnceCell::const_new();

pub static ECHO_ACCOUNT_SERVICES: OnceCell<AccountService> = OnceCell::const_new();
pub static ECHO_SERVICES: OnceCell<Wrapper> = OnceCell::const_new();

pub struct Common<'a> {
    pub db: &'a EchoDatabase<'a>,
    pub cache: &'a EchoCache<'a>,
    pub que: &'a EchoQue<'a>,
    pub services: &'a Wrapper<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        register_subscriber("rustaccount.log");
        info!("starting account services");
        setup().await;
        Self {
            db: ECHO_DB.get().unwrap(),
            cache: ECHO_CACHE.get().unwrap(),
            que: ECHO_QUEUES.get().unwrap(),
            services: ECHO_SERVICES.get().unwrap(),
        }
    }
}

async fn setup() {
    let postgres = PostgresConfig::new().connect().await.unwrap();
    ECHO_POOL.get_or_init(|| async { postgres }).await;
    ECHO_POSTGRES
        .get_or_init(|| async { DB::new(ECHO_POOL.get().unwrap()) })
        .await;

    ECHO_ACCOUNT_STORE
        .get_or_init(|| async {
            AccountStore::new(new_account_table(ECHO_POSTGRES.get().unwrap()))
        })
        .await;
    ECHO_ACCOUNT_INFO_STORE
        .get_or_init(|| async {
            AccountInfoStore::new(new_account_info_table(ECHO_POSTGRES.get().unwrap()))
        })
        .await;
    ECHO_BASIC_ACCOUNT_INFO_STORE
        .get_or_init(|| async {
            BasicAccountInfoStore::new(new_basic_account_info_table(&ECHO_POSTGRES.get().unwrap()))
        })
        .await;
    ECHO_MANAGED_ACCOUNT_INFO_STORE
        .get_or_init(|| async {
            ManagedAccountInfoStore::new(new_managed_account_info_table(
                &ECHO_POSTGRES.get().unwrap(),
            ))
        })
        .await;
    ECHO_SIGNUP_VERIFICATION_STORE
        .get_or_init(|| async {
            SignupVerificationStore::new(new_signup_verification_table(
                &ECHO_POSTGRES.get().unwrap(),
            ))
        })
        .await;
    ECHO_DB
        .get_or_init(|| async {
            EchoDatabase::new(
                ECHO_ACCOUNT_STORE.get().unwrap(),
                ECHO_ACCOUNT_INFO_STORE.get().unwrap(),
                ECHO_BASIC_ACCOUNT_INFO_STORE.get().unwrap(),
                ECHO_MANAGED_ACCOUNT_INFO_STORE.get().unwrap(),
                ECHO_SIGNUP_VERIFICATION_STORE.get().unwrap(),
            )
        })
        .await;

    let redis_client = RedisConfig::new().connect().unwrap();
    ECHO_CLIENT
        .get_or_init(|| async { redis_client.clone() })
        .await;
    ECHO_REDIS
        .get_or_init(|| async { Cache::new(ECHO_CLIENT.get().unwrap()) })
        .await;
    ECHO_ACCOUNT_CACHE
        .get_or_init(|| async { AccountCache::new(ECHO_REDIS.get().unwrap()) })
        .await;
    ECHO_CACHE
        .get_or_init(|| async { EchoCache::new(ECHO_ACCOUNT_CACHE.get().unwrap()) })
        .await;

    let rabbit_connection = RabbitConfig::new().connect().await.unwrap();
    ECHO_CONNECTION
        .get_or_init(|| async { rabbit_connection })
        .await;
    ECHO_QUEUE
        .get_or_init(|| async { Que::new(ECHO_CONNECTION.get().unwrap()) })
        .await;
    ECHO_EMAIL_QUEUE
        .get_or_init(|| async { EmailQue::new(ECHO_QUEUE.get().unwrap()) })
        .await;
    ECHO_EMAIL_CHANNEL
        .get_or_init(|| async {
            ECHO_EMAIL_QUEUE
                .get()
                .unwrap()
                .create_email_channel()
                .await
                .unwrap()
        })
        .await;
    ECHO_QUEUES
        .get_or_init(|| async {
            EchoQue::new(
                ECHO_EMAIL_QUEUE.get().unwrap(),
                ECHO_EMAIL_CHANNEL.get().unwrap(),
            )
        })
        .await;

    ECHO_ACCOUNT_SERVICES
        .get_or_init(|| async {
            AccountService::new(
                ECHO_DB.get().unwrap(),
                ECHO_CACHE.get().unwrap(),
                ECHO_QUEUES.get().unwrap(),
            )
        })
        .await;
    ECHO_SERVICES
        .get_or_init(|| async { Wrapper::new(ECHO_ACCOUNT_SERVICES.get().unwrap()) })
        .await;
}
