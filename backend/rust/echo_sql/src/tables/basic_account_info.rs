use crate::{basic::ModelBuilder, generic::UUID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct BasicAccountInfo {
    #[serde(rename = "id")]
    pub id: UUID,
    #[serde(rename = "recovery_key")]
    pub recovery_key: UUID,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl BasicAccountInfo {
    pub fn new(
        id: UUID,
        recovery_key: UUID,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        BasicAccountInfo {
            id,
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
