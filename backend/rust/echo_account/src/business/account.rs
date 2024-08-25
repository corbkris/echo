pub struct Account {
    pub id: String,
    pub email: String,
    pub password: String,
    pub days_active: Option<i32>,
    pub verified: Option<bool>,
}

impl Account {
    pub fn new(
        id: String,
        email: String,
        password: String,
        days_active: Option<i32>,
        verified: Option<bool>,
    ) -> Self {
        Account {
            id,
            email,
            password,
            days_active,
            verified,
        }
    }
}
