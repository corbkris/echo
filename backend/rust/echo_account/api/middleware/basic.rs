use crate::middleware::error::ApiError;
use hyper::{Body, Request, Response, StatusCode};
use routerify::prelude::*;

pub async fn logger_handler(req: Request<Body>) -> Result<Request<Body>, ApiError> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

pub async fn handler_404(_: Request<Body>) -> Result<Response<Body>, ApiError> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page Not Found"))
        .unwrap())
}
