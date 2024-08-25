use crate::basic::ModelBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Product {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Product {
    pub fn new(
        id: String,
        name: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Product {
            id,
            name,
            created_at,
            updated_at,
        }
    }
}

impl ModelBuilder for Product {
    fn table_name(&self) -> String {
        return String::from("products");
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        return json!(&self);
    }
}
