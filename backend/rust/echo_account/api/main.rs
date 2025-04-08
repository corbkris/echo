pub mod accounts;
pub mod health;
pub mod middleware;

use accounts::controller::{basic_signup, managed_signup, send_managed_signup_code, AccountState};
use echo_account::assembly::setup::Common;
use health::health_check;
use hyper::Server;
use middleware::{
    basic::logger_handler,
    error::{error_handler, handler_404},
};
use routerify::{Middleware, Router, RouterService};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    let common = Common::new().await;

    info!("starting account server");
    let router = RouterService::new(
        Router::builder()
            .middleware(Middleware::pre(logger_handler))
            .get("/health_check", health_check)
            .scope(
                "/accounts",
                Router::builder()
                    .data(AccountState::new(common.services.account_service))
                    .post("/signup/basic", basic_signup)
                    .post("/signup/managed", send_managed_signup_code)
                    .post("/signup/managed/:code", managed_signup)
                    .err_handler(error_handler)
                    .build()
                    .unwrap(),
            )
            .any(handler_404)
            .build()
            .unwrap(),
    )
    .unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(router);

    info!("App is running on: {}", addr);
    let server_future = tokio::spawn(server);

    // Graceful shutdown on Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down gracefully...");
            // Drop or close resources here, if needed
            // For example: close DB connections, cache clients, etc.
            // Since you are using LazyLock, the resources should be dropped when the program exits
        },
        _ = server_future => {
            info!("Server exited.");
        },
    }
}
