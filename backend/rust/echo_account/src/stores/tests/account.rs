use crate::assembly::setup::Common;
use crate::stores::account::Account;

impl Common {
    pub async fn create_account(&mut self) -> Account {
        match self
            .db
            .accounts
            .insert(&Account::new(
                "".to_string(),
                fakeit::internet::username(),
                fakeit::password::generate(true, true, true, 8),
                None,
                None,
            ))
            .await
        {
            Ok(account) => account,
            Err(err) => panic!("error creating account: {}", err),
        }
    }
}
