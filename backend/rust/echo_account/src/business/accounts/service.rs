use crate::caches::{account::Account as RedisAccount, wrapper::EchoCache};
use crate::queues::{email::EmailSigup, wrapper::EchoQue};
use crate::stores::{
    account::{StoreComparisonOperator, StoreConditionalOperator},
    wrapper::EchoDatabase,
};

use crate::business::{
    account::Account,
    accounts::conversion::{marshal, unmarshal},
};

use bcrypt::{hash, DEFAULT_COST};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;

pub struct Service<'a> {
    pub db: &'a EchoDatabase<'a>,
    pub cache: &'a EchoCache<'a>,
    pub que: &'a EchoQue<'a>,
}

impl<'a> Service<'a> {
    pub fn new(db: &'a EchoDatabase, cache: &'a EchoCache, que: &'a EchoQue) -> Self {
        Service { db, cache, que }
    }

    pub async fn signup(&self, email: String, mut password: String) -> Result<String, String> {
        match self.find_by_email(&email).await {
            Ok(_) => return Err("email already exists".to_string()),
            Err(_) => {}
        };

        password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(err) => return Err(err.to_string()),
        };
        let secret_code: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let signup_key = Uuid::new_v4().to_string();

        match self
            .cache
            .accounts
            .set_signup(
                &signup_key,
                &RedisAccount::new(&email, &password, &secret_code),
            )
            .await
        {
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }

        match self
            .que
            .emails
            .publish_email(&EmailSigup::new(email, secret_code.to_string().to_string()))
            .await
        {
            Ok(_) => Ok(signup_key),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn try_signup_code(&self, code: &str, signup_key: &str) -> Result<Account, String> {
        let account = match self.cache.accounts.get_signup(signup_key).await {
            Ok(account) => account,
            Err(err) => return Err(err.to_string()),
        };

        if account.code != code {
            return Err("invalid code".to_string());
        }

        let mut marshaled_account =
            marshal(Account::email_password(account.email, account.password));

        match self.db.accounts.insert(&mut marshaled_account).await {
            None => Ok(unmarshal(marshaled_account)),
            Some(err) => Err(err.to_string()),
        }
    }

    pub async fn login(&self, email: String, mut password: String) -> Result<Account, String> {
        password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(err) => return Err(err.to_string()),
        };
        match self
            .db
            .accounts
            .basic_search_single(
                &marshal(Account::email_password(email, password)),
                StoreComparisonOperator::Equal,
                StoreConditionalOperator::AND,
            )
            .await
        {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Account, String> {
        match self
            .db
            .accounts
            .basic_search_single(
                &marshal(Account::email(email.to_string())),
                StoreComparisonOperator::Equal,
                StoreConditionalOperator::Basic,
            )
            .await
        {
            Ok(account) => return Ok(unmarshal(account)),
            Err(err) => return Err(err.to_string()),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Account, String> {
        match self
            .db
            .accounts
            .basic_search_single(
                &marshal(Account::id(id.to_string())),
                StoreComparisonOperator::Equal,
                StoreConditionalOperator::Basic,
            )
            .await
        {
            Ok(account) => return Ok(unmarshal(account)),
            Err(err) => return Err(err.to_string()),
        }
    }
}
