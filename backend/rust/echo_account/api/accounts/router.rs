use crate::accounts::controller::{signup, State};
use crate::middleware::{authorization::full, basic::handler_404, error::ApiError};
use hyper::Body;
use routerify::{Middleware, Router};

pub fn build_account_subrouter(service: State) -> Router<Body, ApiError> {
    Router::builder()
        .data(service)
        .middleware(Middleware::pre(full))
        .get("/signup/:email/:password", signup)
        .any(handler_404)
        .build()
        .unwrap()
}
