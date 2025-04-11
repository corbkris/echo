use crate::{basic::ModelBuilder, generic::UUID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Account {
    #[serde(rename = "id")]
    pub id: Option<UUID>,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(
        id: Option<UUID>,
        username: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Account {
            id,
            username,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for Account {
    fn table_name(&self) -> String {
        String::from("accounts")
    }

    fn id(&self) -> String {
        match self.id {
            Some(uuid) => format!("'{}'", uuid),
            None => "NULL".to_string(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        json!(&self)
    }
}
