use crate::accounts::controller::{signup, State};
use crate::middleware::error::{handler_404, ApiError};
use hyper::Body;
use routerify::Router;

pub fn build_account_subrouter(service: State) -> Router<Body, ApiError> {
    Router::builder()
        .data(service)
        .get("/signup/:email/:password", signup)
        .any(handler_404)
        .build()
        .unwrap()
}
