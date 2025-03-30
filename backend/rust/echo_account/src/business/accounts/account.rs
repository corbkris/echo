use crate::business::{account::Account, errors::ServiceError};

use super::{account_conv::unmarshal, service::Service};

impl<'a> Service<'a> {
    pub async fn find_account_by_username(&self, username: &str) -> Result<Account, ServiceError> {
        match self.db.accounts.find_by_username(username).await {
            Ok(account) => return Ok(unmarshal(account)),
            Err(err) => return Err(ServiceError::Postgres(err)),
        }
    }
}
