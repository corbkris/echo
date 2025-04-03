use std::char;

use crate::caches::wrapper::EchoCache;
use crate::queues::email::EmailSigup;
use crate::queues::wrapper::EchoQue;
use crate::stores::account::Account;
use crate::stores::account_info::AccountInfo;
use crate::stores::basic_account_info::BasicAccountInfo;
use crate::stores::signup_verification::SignupVerification;
use crate::stores::wrapper::EchoDatabase;
use crate::{business::errors::ServiceError, stores::managed_account_info::ManagedAccountInfo};

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
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

    pub async fn try_signup_code(&self, req_id: Uuid, code: &str) -> Option<ServiceError> {
        let signup_verification = match self
            .db
            .signup_verification
            .find_unexpired_by_id_code(req_id, code)
            .await
        {
            Ok(signup_verification) => signup_verification,
            Err(err) => {
                error!("failed to find signup_verification, {}", err);
                return Some(ServiceError::Postgres(err));
            }
        };

        let mut account = Account {
            username: signup_verification.username,
            ..Default::default()
        };

        if let Some(err) = self.db.accounts.insert(&mut account).await {
            error!("failed to insert account, {}", err);
            return Some(ServiceError::Postgres(err));
        };

        let mut account_info = AccountInfo {
            account_id: account.id.unwrap(),
            password: signup_verification.password,
            ..Default::default()
        };

        if let Some(err) = self.db.account_info.insert(&mut account_info).await {
            error!("failed to insert account_info, {}", err);
            return Some(ServiceError::Postgres(err));
        };

        let mut managed_account_info = ManagedAccountInfo {
            id: account_info.id.unwrap(),
            email: signup_verification.email,
            ..Default::default()
        };

        if let Some(err) = self
            .db
            .managed_account_info
            .insert(&mut managed_account_info)
            .await
        {
            error!("failed to insert managed_account_info, {}", err);
            return Some(ServiceError::Postgres(err));
        };

        None
    }

    //returns request_id
    pub async fn send_managed_signup_verification(
        &self,
        email: &str,
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

        let code: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let mut signup_verification = SignupVerification {
            email: email.to_string(),
            username: username.to_string(),
            password: hashed.to_string(),
            code: code.to_string(),
            expiration: Utc::now() + chrono::Duration::minutes(5),
            ..Default::default()
        };

        if let Some(err) = self
            .db
            .signup_verification
            .insert(&mut signup_verification)
            .await
        {
            error!("failed to insert signup_verification, {}", err);
            return Err(ServiceError::Postgres(err));
        };

        match self
            .que
            .emails
            .publish_email(
                self.que.email_channel,
                &EmailSigup::new(email.to_string(), code.to_string()),
            )
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!("failed to enqueue signup, {}", err);
                return Err(ServiceError::Rabbit(err));
            }
        };

        Ok(signup_verification.id.unwrap().to_string())
    }
}
