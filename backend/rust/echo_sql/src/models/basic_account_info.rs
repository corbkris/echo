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
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    #[serde(rename = "recovery_key")]
    pub recover_key: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}
