use crate::caches::{account::AccountCache, wrapper::EchoCache};
use async_once_cell::OnceCell;
use echo_redis::{connection::Config, generic::Cache};
use redis::Client;

pub static ECHO_CACHE: OnceCell<EchoCache> = OnceCell::new();
pub static ECHO_CLIENT: OnceCell<Client> = OnceCell::new();
pub static ECHO_REDIS: OnceCell<Cache> = OnceCell::new();
pub static ECHO_ACCOUNT_CACHE: OnceCell<AccountCache> = OnceCell::new();

pub struct Common<'a> {
    pub cache: &'a EchoCache<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup().await;
        let cache = ECHO_CACHE.get().unwrap();
        Self { cache }
    }
}

async fn get_client() -> Client {
    Config::new().connect().unwrap()
}

async fn set_client() {
    ECHO_CLIENT.get_or_init(get_client()).await;
}

async fn get_redis<'a>() -> Cache<'a> {
    Cache::new(ECHO_CLIENT.get().unwrap())
}

async fn set_redis() {
    ECHO_REDIS.get_or_init(get_redis()).await;
}

async fn get_account_cache<'a>() -> AccountCache<'a> {
    AccountCache::new(ECHO_REDIS.get().unwrap())
}

async fn set_account_cache() {
    ECHO_ACCOUNT_CACHE.get_or_init(get_account_cache()).await;
}

async fn get_echo_cache<'a>() -> EchoCache<'a> {
    EchoCache::new(ECHO_ACCOUNT_CACHE.get().unwrap())
}

async fn set_echo_cache() {
    ECHO_CACHE.get_or_init(get_echo_cache()).await;
}

pub async fn setup() {
    set_client().await;
    set_redis().await;
    set_account_cache().await;
    set_echo_cache().await;
}
