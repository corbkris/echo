use crate::basic::ModelBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct BasicAccountInfo {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "account_info_id")]
    pub account_info_id: String,
    #[serde(rename = "recovery_key")]
    pub recovery_key: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl BasicAccountInfo {
    pub fn new(
        id: String,
        account_info_id: String,
        recovery_key: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        BasicAccountInfo {
            id,
            account_info_id,
            recovery_key,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for BasicAccountInfo {
    fn table_name(&self) -> String {
        return String::from("basic_account_info");
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
