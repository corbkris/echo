use crate::middleware::error::ApiError;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedSignup {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedSignupAcceptor {
    pub signup: ManagedSignup,
}

pub async fn marshal_managed_signup<'a>(req: Body) -> Result<ManagedSignupAcceptor, ApiError<'a>> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("Failed to read body"));
        }
    };

    match from_slice::<ManagedSignupAcceptor>(&whole_body) {
        Ok(acceptor) => Ok(acceptor),
        Err(err) => {
            error!("{}", err);
            Err(ApiError::Internal("Failed to parse json"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicSignup {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicSignupAcceptor {
    pub signup: BasicSignup,
}

pub async fn marshal_basic_signup<'a>(req: Body) -> Result<BasicSignupAcceptor, ApiError<'a>> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("Failed to read body"));
        }
    };

    match from_slice::<BasicSignupAcceptor>(&whole_body) {
        Ok(acceptor) => Ok(acceptor),
        Err(err) => {
            error!("{}", err);
            Err(ApiError::Internal("Failed to parse json"))
        }
    }
}
