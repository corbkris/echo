use crate::business::account::Account;

impl Account {
    pub fn email_password(email: String, password: String) -> Self {
        Account {
            id: "".to_string(),
            username: "".to_string(),
            email,
            password,
            days_active: None,
            verified: None,
        }
    }

    pub fn email(email: String) -> Self {
        Account {
            id: "".to_string(),
            username: "".to_string(),
            email,
            password: "".to_string(),
            days_active: None,
            verified: None,
        }
    }

    pub fn id(id: String) -> Self {
        Account {
            id,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            days_active: None,
            verified: None,
        }
    }
}
