use crate::middleware::error::ApiError;
use hyper::{Body, Request, Response, StatusCode};
use tracing::{info, instrument};

#[instrument]
pub async fn health_check<'a>(req: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    info!("healthy...");
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}
