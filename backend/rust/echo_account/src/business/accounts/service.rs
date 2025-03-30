use crate::business::errors::ServiceError;
use crate::caches::wrapper::EchoCache;
use crate::queues::wrapper::EchoQue;
use crate::stores::account::Account;
use crate::stores::account_info::AccountInfo;
use crate::stores::basic_account_info::BasicAccountInfo;
use crate::stores::wrapper::EchoDatabase;

use bcrypt::{hash, DEFAULT_COST};
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

    pub async fn basic_signup(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, ServiceError> {
        let hashed = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(err) => {
                error!("failed to hash password,{}", err);
                return Err(ServiceError::Generic(err.to_string()));
            }
        };

        let mut account = Account {
            username: username.to_string(),
            ..Default::default()
        };

        if let Some(err) = self.db.accounts.insert(&mut account).await {
            error!("failed to insert account, {}", err);
            return Err(ServiceError::Postgres(err));
        };

        let mut account_info = AccountInfo {
            account_id: account.id.unwrap(),
            password: hashed,
            ..Default::default()
        };

        if let Some(err) = self.db.account_info.insert(&mut account_info).await {
            error!("failed to insert account_info, {}", err);
            return Err(ServiceError::Postgres(err));
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
            return Err(ServiceError::Postgres(err));
        };
        return Ok(basic_account_info.recovery_key.to_string());
    }
}
