use crate::middleware::error::ApiError;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupAcceptor {
    pub signup: SignupRequest,
}

pub async fn marshal_signup<'a>(req: Body) -> Result<SignupAcceptor, ApiError<'a>> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(_) => return Err(ApiError::Internal("Failed to read body".into())),
    };

    match from_slice::<SignupAcceptor>(&whole_body) {
        Ok(acceptor) => Ok(acceptor),
        Err(_) => Err(ApiError::Internal("Failed to parse json".into())),
    }
}
