use crate::middleware::error::ApiError;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginAcceptor {
    pub login: Login,
}

pub async fn marshal_login<'a>(req: Body) -> Result<LoginAcceptor, ApiError<'a>> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("Failed to read body"));
        }
    };

    match from_slice::<LoginAcceptor>(&whole_body) {
        Ok(acceptor) => Ok(acceptor),
        Err(err) => {
            error!("{}", err);
            Err(ApiError::Internal("Failed to parse json"))
        }
    }
}
