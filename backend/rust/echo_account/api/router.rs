use crate::accounts::{controller::State as account_state, router::build_account_subrouter};
use crate::middleware::{
    authorization::Service as middleware_service,
    basic::{handler_404, handler_error, logger_handler},
};
use echo_account::business::accounts::service::Service as account_service;
use echo_account::business::wrapper::Wrapper;
use hyper::Body;
use routerify::{Error, Middleware, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MasterState {
    pub account_service: Arc<Mutex<account_service>>,
}

impl MasterState {
    pub fn new(services: Wrapper) -> Self {
        MasterState {
            account_service: services.account_service,
        }
    }
}
pub fn build_router(service: MasterState) -> Router<Body, Error> {
    Router::builder()
        .data(middleware_service::new(service.account_service.clone()))
        .middleware(Middleware::pre(logger_handler))
        .scope(
            "/accounts",
            build_account_subrouter(account_state::new(service.account_service.clone())),
        )
        .err_handler(handler_error)
        .any(handler_404)
        .build()
        .unwrap()
}