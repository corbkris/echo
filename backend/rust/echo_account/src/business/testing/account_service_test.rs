#[cfg(test)]
mod tests {
    use crate::assembly::setup::Common;
    use fakeit;

    #[tokio::test]
    async fn test_managed_signup() {
        let common = Common::new().await;
        let email = fakeit::contact::email();
        let username = fakeit::internet::username();

        let req_id = match common
            .services
            .account_service
            .send_managed_signup_verification_code(
                &email,
                &username,
                &fakeit::password::generate(true, true, true, 8),
            )
            .await
        {
            Ok(req_id) => req_id,
            Err(err) => panic!("failed to create signup verification: {}", err),
        };

        let signup_verification = match common.db.signup_verification.find_by_id(req_id).await {
            Ok(signup_verification) => signup_verification,
            Err(err) => panic!("faild to find signup_verification by id: {}", err),
        };

        match common
            .services
            .account_service
            .managed_signup(req_id, &signup_verification.code)
            .await
        {
            Some(err) => panic!("failed to managed signup: {}", err),
            None => {}
        };

        let actual = match common
            .services
            .account_service
            .find_account_by_email(&email)
            .await
        {
            Ok(actual) => actual,
            Err(err) => panic!("failed to find account by email: {}", err),
        };
        assert_eq!(&actual.username, &username);

        match common
            .services
            .account_service
            .delete_signup_verification_by_req_id(req_id)
            .await
        {
            Some(err) => panic!("failed to delete signup_verification: {}", err),
            None => {}
        };

        let deleted_verification = common.db.signup_verification.find_by_id(req_id).await;
        assert!(deleted_verification.is_err());
    }

    #[tokio::test]
    async fn test_basic_signup() {
        let common = Common::new().await;
        let username = fakeit::internet::username();
        let recovery_key = match common
            .services
            .account_service
            .basic_signup(&username, &fakeit::password::generate(true, true, true, 8))
            .await
        {
            Ok(recovery_key) => recovery_key,
            Err(err) => panic!("basic signup failed: {}", err),
        };
        assert_ne!(&recovery_key.to_string(), "");

        let actual = match common
            .services
            .account_service
            .find_account_by_username(&username)
            .await
        {
            Ok(account) => account,
            Err(err) => panic!("failed to find account by username: {}", err),
        };

        assert_eq!(&actual.username, &username)
    }
}
