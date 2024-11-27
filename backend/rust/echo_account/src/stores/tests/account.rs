use crate::assembly::setup::Common;
use crate::stores::account::Account;

impl Common {
    pub async fn create_account(&mut self) -> Account {
        let mut account = Account::new(
            "".to_string(),
            fakeit::internet::username(),
            fakeit::password::generate(true, true, true, 8),
            None,
            None,
        );
        match self.db.accounts.insert(&mut account).await {
            None => account,
            Some(err) => panic!("error creating account: {}", err),
        }
    }
}
