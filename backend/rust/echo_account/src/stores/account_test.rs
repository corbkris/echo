#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::assembly::setup::Common;
    use echo_sql::basic::{ComparisonOperator, ConditonalOperator, ModelBuilder};
    use echo_sql::models::account::Account;
    use fakeit::{internet, password};

    async fn test_setup_common() -> Common {
        let common = match Common::new().await {
            Ok(common) => common,
            Err(err) => {
                panic!("{}", err)
            }
        };
        return common;
    }

    #[tokio::test]
    async fn test_user_insert() {
        let common = test_setup_common().await;

        let mut accounts = common.db.accounts;
        let mut account = Account::new(
            "".to_string(),
            internet::username(),
            password::generate(true, true, true, 8),
            None,
            None,
        );
        let result = accounts.insert(&mut account).await;
        assert!(result.is_none());
        assert_ne!(account.id, "");
    }

    #[tokio::test]
    async fn test_user_update() {
        let updated_password = RefCell::new(password::generate(true, true, true, 8));
        let mut common = test_setup_common().await;
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
        let mut common = test_setup_common().await;
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
        let mut common = test_setup_common().await;
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
