use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResponse {
    pub username: String,
    pub recovery_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupPresenter {
    pub signup: SignupResponse,
}
