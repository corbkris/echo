use uuid::Uuid;

use crate::business::errors::ServiceError;

use super::service::Service;

impl<'a> Service<'a> {
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
