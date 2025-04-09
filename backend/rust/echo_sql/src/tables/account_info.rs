use crate::{basic::ModelBuilder, generic::UUID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")] // Optional: Match enum variants to lowercase strings for JSON
#[sqlx(type_name = "account_type", rename_all = "lowercase")] // Match the PostgreSQL enum type name
pub enum AccountType {
    Basic,
    Managed,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Basic // Specify the default variant
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct AccountInfo {
    #[serde(rename = "id")]
    pub id: Option<UUID>,
    #[serde(rename = "account_id")]
    pub account_id: UUID,
    #[serde(rename = "password")]
    pub password: String,
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
        id: Option<UUID>,
        account_id: UUID,
        password: String,
        account_type: AccountType,
        days_active: Option<i32>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        AccountInfo {
            id,
            account_id,
            password,
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
        match self.id {
            Some(uuid) => format!("'{}'", uuid),
            None => "NULL".to_string(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
