use echo_sql::generic::UUID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Account {
    pub id: UUID,
    pub username: String,
    pub email: String,
    pub days_active: Option<i32>,
    pub verified: Option<bool>,
}

impl Account {
    pub fn new(
        id: UUID,
        username: String,
        email: String,
        days_active: Option<i32>,
        verified: Option<bool>,
    ) -> Self {
        Account {
            id,
            username,
            email,
            days_active,
            verified,
        }
    }
}
