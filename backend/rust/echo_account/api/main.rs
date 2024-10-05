pub mod accounts;
pub mod middleware;
pub mod router;

use crate::router::{build_router, MasterState};
use echo_account::assembly::setup::Common;
use hyper::Server;
use routerify::RouterService;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let common = Common::new().await.unwrap();
    let router = build_router(MasterState::new(common.services));

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
