use serde::Serialize;
#[derive(Serialize)]
pub struct EmailSignup {
    email: String,
    code: String,
}

impl EmailSignup {
    pub fn new(email: String, code: String) -> Self {
        Self { email, code }
    }
}
