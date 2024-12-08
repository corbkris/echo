pub mod accounts;
pub mod middleware;

use accounts::controller::AccountState;
use echo_account::{
    business::{accounts::service::Service as AccountService, wrapper::Wrapper},
    caches::{account::AccountCache, wrapper::EchoCache},
    queues::{email::EmailQue, wrapper::EchoQue},
    stores::{
        account::AccountStore, account_info::AccountInfoStore,
        basic_account_info::BasicAccountInfoStore, managed_account_info::ManagedAccountInfoStore,
        wrapper::EchoDatabase,
    },
};
use echo_rabbit::{
    connection::{Config as RabbitConfig, RabbitConnection},
    generic::{Que, RabbitChannel},
};
use echo_redis::{
    connection::{Config as CacheConfig, RedisClient},
    generic::Cache,
};
use echo_sql::{
    connection::{Config as DBConfig, PostgresPool},
    generic::DB,
};
use hyper::Server;
use middleware::error::{error_handler, handler_404};
use routerify::{Router, RouterService};
use std::{net::SocketAddr, sync::LazyLock};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

    static POSTGRES_POOL: LazyLock<PostgresPool> =
        LazyLock::new(|| RT.block_on(async { DBConfig::new().connect().await.unwrap() }));
    static POSTGRES_DB: LazyLock<DB> = LazyLock::new(|| DB::new(&POSTGRES_POOL));
    static ACCOUNT_STORE: LazyLock<AccountStore> =
        LazyLock::new(|| AccountStore::new(&POSTGRES_DB));
    static ACCOUNT_INFO_STORE: LazyLock<AccountInfoStore> =
        LazyLock::new(|| AccountInfoStore::new(&POSTGRES_DB));
    static BASIC_ACCOUNT_INFO_STORE: LazyLock<BasicAccountInfoStore> =
        LazyLock::new(|| BasicAccountInfoStore::new(&POSTGRES_DB));
    static MANAGED_ACCOUNT_INFO_STORE: LazyLock<ManagedAccountInfoStore> =
        LazyLock::new(|| ManagedAccountInfoStore::new(&POSTGRES_DB));
    static DB: LazyLock<EchoDatabase> = LazyLock::new(|| {
        EchoDatabase::new(
            &ACCOUNT_STORE,
            &ACCOUNT_INFO_STORE,
            &BASIC_ACCOUNT_INFO_STORE,
            &MANAGED_ACCOUNT_INFO_STORE,
        )
    });

    //cache
    static REDIS_CLIENT: LazyLock<RedisClient> =
        LazyLock::new(|| CacheConfig::new().connect().unwrap());
    static REDIS_CACHE: LazyLock<Cache> = LazyLock::new(|| Cache::new(&REDIS_CLIENT));
    static ACCOUNT_CACHE: LazyLock<AccountCache> =
        LazyLock::new(|| AccountCache::new(&REDIS_CACHE));
    static CACHE: LazyLock<EchoCache> = LazyLock::new(|| EchoCache::new(&ACCOUNT_CACHE));

    //rabbit
    static RABBIT_CONNECTION: LazyLock<RabbitConnection> =
        LazyLock::new(|| RT.block_on(async { RabbitConfig::new().connect().await.unwrap() }));
    static RABBIT_QUE: LazyLock<Que> = LazyLock::new(|| Que::new(&RABBIT_CONNECTION));
    static EMAIL_CHANNEL: LazyLock<RabbitChannel> = LazyLock::new(|| {
        RT.block_on(async { RABBIT_QUE.create_channel("email_channel").await.unwrap() })
    });
    static EMAIL_QUE: LazyLock<EmailQue> =
        LazyLock::new(|| EmailQue::new(&RABBIT_QUE, &EMAIL_CHANNEL));
    static QUE: LazyLock<EchoQue> = LazyLock::new(|| EchoQue::new(&EMAIL_QUE));

    //services
    static ACCOUNT_SERVICE: LazyLock<AccountService> =
        LazyLock::new(|| AccountService::new(&DB, &CACHE, &QUE));
    static SERVICES: LazyLock<Wrapper> = LazyLock::new(|| Wrapper::new(&ACCOUNT_SERVICE));

    let router = RouterService::new(
        Router::builder()
            .scope(
                "/accounts",
                Router::builder()
                    .data(AccountState::new(SERVICES.account_service))
                    .any(handler_404)
                    .build()
                    .unwrap(),
            )
            .err_handler(error_handler)
            .any(handler_404)
            .build()
            .unwrap(),
    )
    .unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(router);

    println!("App is running on: {}", addr);
    let server_future = tokio::spawn(server);

    // Graceful shutdown on Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down gracefully...");
            // Drop or close resources here, if needed
            // For example: close DB connections, cache clients, etc.
            // Since you are using LazyLock, the resources should be dropped when the program exits
        },
        _ = server_future => {
            println!("Server exited.");
        },
    }
}
