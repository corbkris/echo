#[cfg(test)]
mod tests {

    use echo_sql::tables::account_info::AccountType;
    use fakeit::{internet, password};

    use crate::assembly::setup::Common;
    use crate::stores::account_info::AccountInfo;
    use crate::stores::managed_account_info::ManagedAccountInfo;

    #[tokio::test]
    async fn test_account_find_by_email_password() {
        let common = Common::new().await;
        common.test_create_account().await;
        let expected = common.test_create_account().await;
        let account_info = &mut AccountInfo {
            account_id: expected.id.unwrap(),
            password: password::generate(true, true, true, 8),
            account_type: AccountType::Basic,
            ..Default::default()
        };

        match common.db.account_info.insert(account_info).await {
            None => {}
            Some(err) => panic!("error creating account_info: {}", err),
        };

        let email = &internet::username();

        let managed_account_info = &mut ManagedAccountInfo {
            id: account_info.id.unwrap(),
            email: email.to_string(),
            ..Default::default()
        };

        match common
            .db
            .managed_account_info
            .insert(managed_account_info)
            .await
        {
            None => {}
            Some(err) => panic!("error creating managed_account_info: {}", err),
        };

        let actual = common
            .db
            .account_info
            .find_by_email(email)
            .await
            .unwrap_or_else(|e| panic!("failed to search by email: {}", e));
        assert_eq!(expected.id.unwrap(), actual.account_id);
    }

    #[tokio::test]
    async fn test_account_info_find_by_username() {
        let common = Common::new().await;
        let expected = common.test_create_account().await;
        let account_info = &mut AccountInfo {
            account_id: expected.id.unwrap(),
            password: password::generate(true, true, true, 8),
            account_type: AccountType::Basic,
            ..Default::default()
        };

        match common.db.account_info.insert(account_info).await {
            None => {}
            Some(err) => panic!("error creating account_info: {}", err),
        };

        let actual = match common
            .db
            .account_info
            .find_by_username(&expected.username)
            .await
        {
            Ok(actual) => actual,
            Err(err) => panic!("failed to find account_info by username: {}", err),
        };
        assert_eq!(expected.id.unwrap(), actual.account_id);
    }
}
