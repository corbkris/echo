use crate::basic::ModelBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "account_type")] // Match the PostgreSQL enum type name
#[serde(rename_all = "lowercase")] // Optional: Match enum variants to lowercase strings for JSON
pub enum AccountType {
    Basic,
    Managed,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_type")]
    pub account_type: AccountType,
    #[serde(rename = "days_active")]
    pub days_active: Option<i32>,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl AccountInfo {
    pub fn new(
        id: String,
        account_id: String,
        account_type: AccountType,
        days_active: Option<i32>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        AccountInfo {
            id,
            account_id,
            account_type,
            days_active,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for AccountInfo {
    fn table_name(&self) -> String {
        return String::from("account_info");
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
