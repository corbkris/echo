use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
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

pub fn generate_account_token(user_id: &str) -> Result<String, &'static str> {
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

    // Signing key
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret_key").map_err(|_e| "Invalid key")?;

    // Sign the token with the custom claims
    let signed_token = claims.sign_with_key(&key).map_err(|_e| "Sign failed")?;

    Ok(signed_token)
}

pub fn verify_account_token(token: &str) -> Result<String, &'static str> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret_key").map_err(|_e| "Invalid key")?;

    // Verify and deserialize into CustomClaims instead of RegisteredClaims
    let claims: AccountToken =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Parse failed")?;

    // Now you can access both the registered and custom claims
    let _ = claims.registered.subject.ok_or("Missing subject")?;

    // Optionally, check custom claims like role or expiration (exp)
    if claims.role != "basic" {
        return Err("Unauthorized role");
    }

    // Example check for expiration time (not secure, this is for illustration)
    if claims.exp < Utc::now().timestamp() as usize {
        return Err("Token expired");
    }

    let account_id = claims.id;

    Ok(account_id)
}
