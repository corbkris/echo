use hmac::Hmac;
use jwt::{Error, RegisteredClaims, SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize)]
pub struct AccountToken {
    #[serde(flatten)]
    registered: RegisteredClaims, // Embedding standard claims
    id: String,
    role: String, // Custom claim
    exp: usize,   // Expiration time in seconds since epoch
}

pub fn generate_auth_token(user_id: &str, key: Hmac<Sha256>) -> Result<String, Error> {
    let claims = AccountToken {
        registered: RegisteredClaims {
            issuer: Some("echo.com".into()),
            subject: Some("account_token".into()),
            ..Default::default()
        },
        id: user_id.to_string(),
        role: "basic".to_string(), // Example custom claim
        exp: 1692525600,           // Example expiration time (Unix timestamp)
    };

    // Sign the token with the custom claims
    let signed_token = claims.sign_with_key(&key)?;

    Ok(signed_token)
}

pub fn parse_auth_token(token: &str, key: Hmac<Sha256>) -> Result<AccountToken, Error> {
    let claims: AccountToken = VerifyWithKey::verify_with_key(token, &key)?;
    Ok(claims)
}
