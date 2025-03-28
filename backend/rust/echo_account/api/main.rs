pub mod accounts;
pub mod middleware;

use accounts::controller::AccountState;
use echo_account::assembly::setup::Common;
use hyper::Server;
use middleware::error::{error_handler, handler_404};
use routerify::{Router, RouterService};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    let common = Common::new().await;

    info!("starting account server");
    let router = RouterService::new(
        Router::builder()
            .scope(
                "/accounts",
                Router::builder()
                    .data(AccountState::new(common.services.account_service))
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
