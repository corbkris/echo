use crate::middleware::error::ApiError;
use hyper::{Body, Request};
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
