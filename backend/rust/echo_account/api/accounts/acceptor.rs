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

pub async fn marshal_signup(req: Body) -> Result<SignupRequest, ApiError> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(_) => return Err(ApiError::Generic("Failed to read body".into())),
    };

    match from_slice::<SignupRequest>(&whole_body) {
        Ok(signup_data) => Ok(signup_data),
        Err(_) => Err(ApiError::Generic("Failed to parse json".into())),
    }
}
