use crate::business::errors::ServiceError;
use crate::caches::{account::Account as RedisAccount, wrapper::EchoCache};
use crate::queues::email::EmailSigup;
use crate::queues::wrapper::EchoQue;
use crate::stores::account::Account;
use crate::stores::account_info::AccountInfo;
use crate::stores::basic_account_info::BasicAccountInfo;
use crate::stores::wrapper::EchoDatabase;

use bcrypt::{hash, DEFAULT_COST};
use echo_sql::generic::PostgresError;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tracing::error;
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

    pub async fn basic_signup(&self, username: &str, mut password: String) {
        match self
            .db
            .accounts
            .find_by_username(username.to_string())
            .await
        {
            Ok(_) => {
                error!("username taken");
                return;
            }
            Err(err) => {
                if !matches!(err, PostgresError::RowNotFound) {
                    error!("failed to search for account");
                    return;
                }
            }
        };

        password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(err) => {
                error!("failed to hash password,{}", err);
                return;
            }
        };

        let mut account = Account {
            username: username.to_string(),
            ..Default::default()
        };

        if let Some(err) = self.db.accounts.insert(&mut account).await {
            error!("failed to insert account, {}", err);
            return;
        };

        let mut account_info = AccountInfo {
            account_id: account.id.unwrap(),
            password,
            ..Default::default()
        };

        if let Some(err) = self.db.account_info.insert(&mut account_info).await {
            error!("failed to insert account_info, {}", err);
            return;
        };

        let mut basic_account_info = BasicAccountInfo {
            id: account_info.id.unwrap(),
            recovery_key: Uuid::new_v4(),
            ..Default::default()
        };

        if let Some(err) = self
            .db
            .basic_account_info
            .insert(&mut basic_account_info)
            .await
        {
            error!("failed to insert basic_account_info, {}", err);
            return;
        };
    }

    pub async fn signup(
        &self,
        email: String,
        mut password: String,
    ) -> Result<String, ServiceError> {
        password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(err) => return Err(ServiceError::Generic(err.to_string())),
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
            Err(err) => return Err(ServiceError::Redis(err)),
        }

        match self
            .que
            .emails
            .publish_email(
                &self.que.email_channel,
                &EmailSigup::new(email, secret_code.to_string().to_string()),
            )
            .await
        {
            Ok(_) => Ok(signup_key),
            Err(err) => Err(ServiceError::Rabbit(err)),
        }
    }
}
