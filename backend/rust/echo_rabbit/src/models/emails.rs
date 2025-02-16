use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct EmailSignup {
    email: String,
    key: String,
}

impl EmailSignup {
    pub fn new(email: String, key: String) -> Self {
        Self { email, key }
    }

    pub fn to_payload(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
