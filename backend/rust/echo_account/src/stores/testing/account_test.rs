#[cfg(test)]
mod tests {

    use echo_sql::basic::{ComparisonOperator, ConditonalOperator, ModelBuilder};
    use echo_sql::models::account::Account;
    use std::cell::RefCell;

    use crate::stores::testing::test::Common;
    use fakeit::{internet, password};

    #[tokio::test]
    async fn test_user_insert() {
        let common = Common::new().await;
        let mut account = Account::new(
            "".to_string(),
            internet::username(),
            password::generate(true, true, true, 8),
            None,
            None,
        );
        let result = common.db.accounts.insert(&mut account).await;
        assert!(result.is_none());
        assert_ne!(account.id, "");
    }

    #[tokio::test]
    async fn test_user_update() {
        let common = Common::new().await;
        let updated_password = RefCell::new(password::generate(true, true, true, 8));
        let mut account = common.create_account().await;
        account.password = updated_password.borrow().to_string();
        let result = common.db.accounts.update(&mut account).await;
        assert!(result.is_none());
        assert_eq!(updated_password.borrow().to_string(), account.password);

        let result = common
            .db
            .accounts
            .basic_search_single(&account, ComparisonOperator::Equal, ConditonalOperator::AND)
            .await;
        assert!(result.is_ok());
        assert_eq!(
            updated_password.borrow().to_string(),
            result.unwrap().password
        );
    }

    #[tokio::test]
    async fn test_user_delete() {
        let common = Common::new().await;
        let account = common.create_account().await;
        assert!(common.db.accounts.delete(&account).await.is_ok());
        let result = common
            .db
            .accounts
            .basic_search_single(
                &Account::new(account.id(), "".to_string(), "".to_string(), None, None),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            )
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_user_delete_full() {
        let common = Common::new().await;
        let account = common.create_account().await;
        assert!(common.db.accounts.delete(&account).await.is_ok());
        let result = common
            .db
            .accounts
            .basic_search_single(&account, ComparisonOperator::Equal, ConditonalOperator::AND)
            .await;
        assert!(result.is_err())
    }
}
