use crate::middleware::error::ApiError;
use routerify::RouteParams;
use tracing::{error, instrument};

#[instrument]
pub fn parse_route_param<'a>(params: &RouteParams, key: &'a str) -> Result<String, ApiError<'a>> {
    match params.get(key) {
        Some(param) => Ok(param.to_string()),
        None => {
            error!("mssing route param: {}", key);
            Err(ApiError::BadRequestFmt("missing route param", key))
        }
    }
}
