use crate::stores::testing::test::Common;
use echo_sql::models::account::Account;

impl<'a> Common<'a> {
    pub async fn create_account(&self) -> Account {
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
