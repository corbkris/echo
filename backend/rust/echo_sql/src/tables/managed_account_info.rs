use crate::basic::ModelBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ManagedAccountInfo {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "account_info_id")]
    pub account_info_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl ManagedAccountInfo {
    pub fn new(
        id: String,
        account_info_id: String,
        email: String,
        phone: String,
        verified: Option<bool>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        ManagedAccountInfo {
            id,
            account_info_id,
            email,
            phone,
            verified,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for ManagedAccountInfo {
    fn table_name(&self) -> String {
        return String::from("managed_account_info");
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
