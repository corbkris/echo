use crate::accounts::controller::{signup, State};
use crate::middleware::authorization::full;
use hyper::Body;
use routerify::Error;
use routerify::{Middleware, Router};

pub fn build_account_subrouter(service: State) -> Router<Body, Error> {
    Router::builder()
        .data(service)
        .middleware(Middleware::pre(full))
        .get("/signup/:email/:password", signup)
        .build()
        .unwrap()
}
