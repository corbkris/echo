use crate::{assembly::setup::Common, stores::account::Account};

impl<'a> Common<'a> {
    pub async fn create_account(&self) -> Account {
        let mut account = Account::new(None, fakeit::internet::username(), None, None);
        match self.db.accounts.insert(&mut account).await {
            None => account,
            Some(err) => panic!("error creating account: {}", err),
        }
    }
}
