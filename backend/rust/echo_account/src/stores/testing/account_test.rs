#[cfg(test)]
mod tests {

    use echo_sql::basic::{ComparisonOperator, ConditonalOperator};

    use echo_sql::tables::account_info::AccountType;
    use fakeit::{internet, password};

    use crate::assembly::setup::Common;
    use crate::stores::{
        account::Account, account_info::AccountInfo, managed_account_info::ManagedAccountInfo,
    };

    #[tokio::test]
    async fn test_account_find_by_username_password() {
        let common = Common::new().await;
        let expected = common.test_create_account().await;
        let actual = common
            .db
            .accounts
            .find_by_username(&expected.username)
            .await
            .expect("failed to search by username");
        assert_eq!(&expected.username, &actual.username);
    }

    #[tokio::test]
    async fn test_account_find_by_username() {
        let common = Common::new().await;
        common.test_create_account().await;
        let expected = common.test_create_account().await;
        let actual = common
            .db
            .accounts
            .find_by_username(&expected.username)
            .await
            .expect("failed to search by username");
        assert_eq!(&expected.username, &actual.username);
    }

    #[tokio::test]
    async fn test_account_find_by_email() {
        let common = Common::new().await;

        common.test_create_account().await;
        let expected = common.test_create_account().await;

        let account_info = &mut AccountInfo {
            account_id: expected.id.unwrap(),
            password: password::generate(true, true, true, 8),
            account_type: AccountType::Managed,
            ..Default::default()
        };

        match common.db.account_info.insert(account_info).await {
            None => {}
            Some(err) => panic!("error creating account_info: {}", err),
        };

        let managed_account_info = &mut ManagedAccountInfo {
            id: account_info.id.unwrap(),
            email: internet::username(),
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
            .accounts
            .find_by_email(&managed_account_info.email)
            .await
            .expect("failed to find account by email");
        assert_eq!(&expected.username, &actual.username);
    }

    #[tokio::test]
    async fn test_user_insert() {
        let common = Common::new().await;
        let mut account = Account::new(None, internet::username(), None, None);
        let result = common.db.accounts.insert(&mut account).await;
        assert!(result.is_none());
        assert_ne!(account.id, None);
        let result = common.db.accounts.update(&mut account).await;
        assert!(result.is_none());
        assert_ne!(account.id, None);
    }

    #[tokio::test]
    async fn test_user_update() {
        let common = Common::new().await;
        let mut account = common.test_create_account().await;
        let result = common.db.accounts.update(&mut account).await;
        assert!(result.is_none());

        let result = common
            .db
            .accounts
            .search(&account, ComparisonOperator::Equal, ConditonalOperator::AND)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_delete() {
        let common = Common::new().await;
        let account = common.test_create_account().await;
        assert!(common.db.accounts.delete(&account).await.is_ok());
        let result = common
            .db
            .accounts
            .search(
                &Account::new(account.id, "".to_string(), None, None),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            )
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_user_delete_full() {
        let common = Common::new().await;
        let account = common.test_create_account().await;
        assert!(common.db.accounts.delete(&account).await.is_ok());
        let result = common
            .db
            .accounts
            .search(&account, ComparisonOperator::Equal, ConditonalOperator::AND)
            .await;
        assert!(result.is_err())
    }
}
