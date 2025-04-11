use crate::{basic::ModelBuilder, generic::UUID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct ManagedAccountInfo {
    #[serde(rename = "id")]
    pub id: UUID,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    #[serde(rename = "verified")]
    pub verified: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl ManagedAccountInfo {
    pub fn new(
        id: UUID,
        email: String,
        phone: Option<String>,
        verified: Option<bool>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        ManagedAccountInfo {
            id,
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
        String::from("managed_account_info")
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        json!(&self)
    }
}
