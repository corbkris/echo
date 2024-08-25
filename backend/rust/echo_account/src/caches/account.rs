use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub code: String,
}

impl Account {
    pub fn new(email: &str, password: &str, code: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            code: code.to_string(),
        }
    }
}
