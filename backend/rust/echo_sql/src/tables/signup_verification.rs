use crate::{basic::ModelBuilder, generic::UUID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct SignupVerification {
    #[serde(rename = "id")]
    pub id: Option<UUID>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "expiration")]
    pub expiration: DateTime<Utc>,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl SignupVerification {
    pub fn new(
        id: Option<UUID>,
        code: String,
        email: String,
        username: String,
        password: String,
        expiration: DateTime<Utc>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        SignupVerification {
            id,
            code,
            email,
            username,
            password,
            expiration,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for SignupVerification {
    fn table_name(&self) -> String {
        return String::from("signup_verification");
    }

    fn id(&self) -> String {
        match self.id {
            Some(uuid) => format!("'{}'", uuid),
            None => "NULL".to_string(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
