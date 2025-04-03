use uuid::Uuid;

use crate::business::{account::Account, errors::ServiceError};

use super::{account_conv::unmarshal, service::Service};

impl<'a> Service<'a> {
    pub async fn find_account_by_username(&self, username: &str) -> Result<Account, ServiceError> {
        match self.db.accounts.find_by_username(username).await {
            Ok(account) => return Ok(unmarshal(account)),
            Err(err) => return Err(ServiceError::Postgres(err)),
        }
    }

    pub async fn find_account_by_email(&self, email: &str) -> Result<Account, ServiceError> {
        match self.db.accounts.find_by_email(email).await {
            Ok(account) => return Ok(unmarshal(account)),
            Err(err) => return Err(ServiceError::Postgres(err)),
        }
    }

    pub async fn delete_signup_verification_by_req_id(&self, id: Uuid) -> Option<ServiceError> {
        let signup_verification = match self.db.signup_verification.find_by_id(id).await {
            Ok(signup_verification) => signup_verification,
            Err(err) => return Some(ServiceError::Postgres(err)),
        };

        if let Err(err) = self
            .db
            .signup_verification
            .delete(&signup_verification)
            .await
        {
            return Some(ServiceError::Postgres(err));
        }

        None
    }
}
