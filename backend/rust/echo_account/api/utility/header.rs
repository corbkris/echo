use std::str::FromStr;

use crate::middleware::error::ApiError;
use hyper::HeaderMap;
use tracing::{error, instrument};
use uuid::Uuid;

#[instrument]
pub fn parse_header<'a>(headers: &HeaderMap, name: &'a str) -> Result<String, ApiError<'a>> {
    let header = match headers.get(name) {
        Some(header) => header,
        None => return Err(ApiError::BadRequestFmt("missing header", name)),
    };

    match header.to_str() {
        Ok(parsed_header) => Ok(parsed_header.to_string()),
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("failed to parse header to sring"));
        }
    }
}

#[instrument]
pub fn parse_header_uuid<'a>(headers: &HeaderMap, name: &'a str) -> Result<Uuid, ApiError<'a>> {
    let header = parse_header(headers, name)?;
    match Uuid::from_str(&header) {
        Ok(uuid) => Ok(uuid),
        Err(err) => {
            error!("{}", err);
            Err(ApiError::Internal("failed to parse header to uuid"))
        }
    }
}
