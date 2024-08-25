use crate::basic::ModelBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Account {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "days_active")]
    pub days_active: Option<i32>,
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(
        id: String,
        email: String,
        password: String,
        days_active: Option<i32>,
        verified: Option<bool>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Account {
            id,
            email,
            password,
            days_active,
            verified,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for Account {
    fn table_name(&self) -> String {
        return String::from("accounts");
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
