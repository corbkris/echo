#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{assembly::setup::Common, stores::signup_verification::SignupVerification};

    #[tokio::test]
    async fn test_signup_verification_find_by_id_code() {
        let common = Common::new().await;

        let expected = &mut SignupVerification {
            code: "1234".to_string(),
            email: fakeit::contact::email(),
            username: fakeit::internet::username(),
            password: fakeit::password::generate(true, true, true, 8),
            expiration: Utc::now(),
            ..Default::default()
        };

        match common.db.signup_verification.insert(expected).await {
            None => {}
            Some(err) => panic!("failed to insert signup_verification: {}", err),
        };

        let actual = match common
            .db
            .signup_verification
            .find_by_id_code(expected.id.unwrap(), "1234")
            .await
        {
            Ok(actual) => actual,
            Err(err) => panic!("failed to find verification: {}", err),
        };
        assert_eq!(&expected.username, &actual.username);
    }

    #[tokio::test]
    async fn test_signup_verification_find_by_id() {
        let common = Common::new().await;

        let expected = &mut SignupVerification {
            code: "1234".to_string(),
            email: fakeit::contact::email(),
            username: fakeit::internet::username(),
            password: fakeit::password::generate(true, true, true, 8),
            expiration: Utc::now(),
            ..Default::default()
        };

        match common.db.signup_verification.insert(expected).await {
            None => {}
            Some(err) => panic!("failed to insert signup_verification: {}", err),
        };

        let actual = match common
            .db
            .signup_verification
            .find_by_id(expected.id.unwrap())
            .await
        {
            Ok(actual) => actual,
            Err(err) => panic!("failed to find verification: {}", err),
        };
        assert_eq!(&expected.username, &actual.username);
    }
}
