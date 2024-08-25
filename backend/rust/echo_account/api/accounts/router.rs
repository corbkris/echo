use crate::accounts::{
    controller::{signup, Service},
    middleware::full,
};
use hyper::Body;
use routerify::{Middleware, Router};
use std::convert::Infallible;

pub fn build_account_subrouter(service: Service) -> Router<Body, Infallible> {
    Router::builder()
        .data(service)
        .middleware(Middleware::pre(full))
        .get("/signup", signup)
        .build()
        .unwrap()
}
