use crate::middleware::error::ApiError;
use hyper::{Body, Request};
use routerify::prelude::*;
use tracing::{info, instrument};

#[instrument]
pub async fn logger_handler<'a>(req: Request<Body>) -> Result<Request<Body>, ApiError<'a>> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}
