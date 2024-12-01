use async_once_cell::OnceCell;
use echo_sql::{
    connection::{Config, PostgresPool},
    generic::DB,
};

use crate::stores::{
    account::AccountStore, account_info::AccountInfoStore,
    basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
    wrapper::EchoDatabase,
};

pub static ECHO_DB: OnceCell<EchoDatabase> = OnceCell::new();
pub static ECHO_POOL: OnceCell<PostgresPool> = OnceCell::new();
pub static ECHO_POSTGRES: OnceCell<DB> = OnceCell::new();
pub static ECHO_ACCOUNT_STORE: OnceCell<AccountStore> = OnceCell::new();
pub static ECHO_ACCOUNT_INFO_STORE: OnceCell<AccountInfoStore> = OnceCell::new();
pub static ECHO_BASIC_ACCOUNT_INFO_STORE: OnceCell<BasicAccountInfoStore> = OnceCell::new();
pub static ECHO_MANAGED_ACCOUNT_STORE: OnceCell<ManagedAccountInfoStore> = OnceCell::new();

pub struct Common<'a> {
    pub db: &'a EchoDatabase<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup().await;
        let db = ECHO_DB.get().unwrap();
        Self { db }
    }
}

async fn get_pool() -> PostgresPool {
    Config::new().connect().await.unwrap()
}

async fn set_pool() {
    ECHO_POOL.get_or_init(get_pool()).await;
}

async fn get_postgres<'a>() -> DB<'a> {
    DB::new(ECHO_POOL.get().unwrap())
}

async fn set_postgres() {
    ECHO_POSTGRES.get_or_init(get_postgres()).await;
}

async fn get_account_store<'a>() -> AccountStore<'a> {
    AccountStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_account_store() {
    ECHO_ACCOUNT_STORE.get_or_init(get_account_store()).await;
}

async fn get_account_info_store<'a>() -> AccountInfoStore<'a> {
    AccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_account_info_store() {
    ECHO_ACCOUNT_INFO_STORE
        .get_or_init(get_account_info_store())
        .await;
}

async fn get_basic_account_info_store<'a>() -> BasicAccountInfoStore<'a> {
    BasicAccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_basic_account_info_store() {
    ECHO_BASIC_ACCOUNT_INFO_STORE
        .get_or_init(get_basic_account_info_store())
        .await;
}

async fn get_managed_account_info_store<'a>() -> ManagedAccountInfoStore<'a> {
    ManagedAccountInfoStore::new(ECHO_POSTGRES.get().unwrap())
}

async fn set_managed_account_info_store() {
    ECHO_MANAGED_ACCOUNT_STORE
        .get_or_init(get_managed_account_info_store())
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
    ECHO_DB.get_or_init(get_db()).await;
}

pub async fn setup() {
    set_pool().await;
    set_postgres().await;
    set_account_store().await;
    set_account_info_store().await;
    set_basic_account_info_store().await;
    set_managed_account_info_store().await;
    set_db().await;
}
