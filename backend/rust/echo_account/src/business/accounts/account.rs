use crate::business::{account::Account, errors::ServiceError};

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
        match self
            .db
            .accounts
            .find_by_username_password(username, password)
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
        match self
            .db
            .accounts
            .find_by_email_password(email, password)
            .await
        {
            Ok(account) => Ok(unmarshal(account)),
            Err(err) => Err(ServiceError::Postgres(err)),
        }
    }
}
