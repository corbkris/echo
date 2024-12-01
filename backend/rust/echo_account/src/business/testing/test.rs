use async_once_cell::OnceCell;

use crate::{
    business::{accounts::service::Service as AccountService, wrapper::Wrapper},
    caches::testing::test::Common as CacheCommon,
    queues::testing::test::Common as QueCommon,
    stores::testing::test::Common as DbCommon,
};

pub static ECHO_SERVICES: OnceCell<Wrapper> = OnceCell::new();
pub static ECHO_ACCOUNT_SERVICES: OnceCell<AccountService> = OnceCell::new();
pub static ECHO_DATABASE: OnceCell<DbCommon> = OnceCell::new();
pub static ECHO_CACHE: OnceCell<CacheCommon> = OnceCell::new();
pub static ECHO_QUE: OnceCell<QueCommon> = OnceCell::new();

pub struct Common<'a> {
    pub services: &'a Wrapper<'a>,
}

impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup().await;
        let services = ECHO_SERVICES.get().unwrap();
        Self { services }
    }
}

async fn get_database<'a>() -> DbCommon<'a> {
    DbCommon::new().await
}

async fn set_database() {
    ECHO_DATABASE.get_or_init(get_database()).await;
}

async fn get_cache<'a>() -> CacheCommon<'a> {
    CacheCommon::new().await
}

async fn set_cache() {
    ECHO_CACHE.get_or_init(get_cache()).await;
}

async fn get_que<'a>() -> QueCommon<'a> {
    QueCommon::new().await
}

async fn set_que() {
    ECHO_QUE.get_or_init(get_que()).await;
}

async fn get_account_service<'a>() -> AccountService<'a> {
    AccountService::new(
        ECHO_DATABASE.get().unwrap().db,
        ECHO_CACHE.get().unwrap().cache,
        ECHO_QUE.get().unwrap().que,
    )
}

async fn set_account_service() {
    ECHO_ACCOUNT_SERVICES
        .get_or_init(get_account_service())
        .await;
}

async fn get_services<'a>() -> Wrapper<'a> {
    Wrapper::new(ECHO_ACCOUNT_SERVICES.get().unwrap())
}

async fn set_services() {
    ECHO_SERVICES.get_or_init(get_services()).await;
}

pub async fn setup() {
    set_database().await;
    set_cache().await;
    set_que().await;
    set_account_service().await;
    set_services().await;
}
