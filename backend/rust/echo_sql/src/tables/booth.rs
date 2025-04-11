use crate::basic::ModelBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Booth {
    #[sqlx(try_from = "Uuid")]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
}

impl Booth {
    pub fn new(id: String, name: String, verified: bool, account_id: Option<String>) -> Self {
        Booth {
            id,
            name,
            account_id,
            verified,
        }
    }
}

impl ModelBuilder for Booth {
    fn table_name(&self) -> String {
        String::from("booths")
    }

    fn id(&self) -> String {
        format!("'{}'", self.id)
    }

    fn to_json(&self) -> serde_json::Value {
        json!(&self)
    }
}
