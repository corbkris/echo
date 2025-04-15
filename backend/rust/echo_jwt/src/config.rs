use std::env;

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct Config {
    pub secret_key: Hmac<Sha256>,
}

impl Config {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let key = env::var("JWT_SECRET_KEY").expect("failed to read jwt env var");
        let secret_key: Hmac<Sha256> =
            Hmac::new_from_slice(key.as_bytes()).expect("failed to create secret key");

        Self { secret_key }
    }
}
