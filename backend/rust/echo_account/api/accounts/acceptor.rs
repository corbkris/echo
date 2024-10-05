use hyper::Body;
use routerify::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn marshal_signup(req: Body) -> Result<SignupRequest, Error> {
    let whole_body = match hyper::body::to_bytes(req).await {
        Ok(whole_body) => whole_body,
        Err(_) => return Err(Error::new("Failed to read body")),
    };

    match from_slice::<SignupRequest>(&whole_body) {
        Ok(signup_data) => Ok(signup_data),
        Err(_) => Err(Error::new("Failed to parse json")),
    }
}
