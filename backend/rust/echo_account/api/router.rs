use crate::accounts::{
    controller::Service as account_controller_service, router::build_account_subrouter,
};
use echo_account::business::accounts::service::Service as account_service;
use echo_account::business::wrapper::Wrapper;
use hyper::Body;
use routerify::Router;
use std::convert::Infallible;

pub struct MasterService {
    pub account_service: account_service,
}

impl MasterService {
    pub fn new(services: Wrapper) -> Self {
        MasterService {
            account_service: services.account_service,
        }
    }
}
pub fn build_router(service: MasterService) -> Router<Body, Infallible> {
    Router::builder()
        .scope(
            "/accounts",
            build_account_subrouter(account_controller_service::new(service.account_service)),
        )
        .build()
        .unwrap()
}
