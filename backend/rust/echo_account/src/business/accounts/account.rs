use crate::business::{account::Account, errors::ServiceError};
use bcrypt::verify;

use super::{account_conv::unmarshal, service::Service};

impl<'a> Service<'a> {
    pub async fn find_account_by_username(&self, username: &str) -> Result<Account, ServiceError> {
        match self.db.accounts.find_by_username(username).await {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(ServiceError::Postgres(err)),
        }
    }

    pub async fn find_account_by_username_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Account, ServiceError> {
        let account_info = match self.db.account_info.find_by_username(username).await {
            Ok(account_info) => account_info,
            Err(err) => return Err(ServiceError::Postgres(err)),
        };
        if !verify(password, &account_info.password).unwrap() {
            return Err(ServiceError::Internal("invalid password"));
        };

        match self
            .db
            .accounts
            .find_by_id_username(account_info.account_id, username)
            .await
        {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(ServiceError::Postgres(err)),
        }
    }

    pub async fn find_account_by_email(&self, email: &str) -> Result<Account, ServiceError> {
        match self.db.accounts.find_by_email(email).await {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(ServiceError::Postgres(err)),
        }
    }

    pub async fn find_account_by_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Account, ServiceError> {
        let account_info = match self.db.account_info.find_by_email(email).await {
            Ok(account_info) => account_info,
            Err(err) => return Err(ServiceError::Postgres(err)),
        };
        if !verify(password, &account_info.password).unwrap() {
            return Err(ServiceError::Internal("invalid password"));
        };

        match self
            .db
            .accounts
            .find_by_id_email(account_info.account_id, email)
            .await
        {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(ServiceError::Postgres(err)),
        }
    }
}
