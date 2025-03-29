#[cfg(test)]
mod tests {

    use echo_sql::basic::{ComparisonOperator, ConditonalOperator};

    use fakeit::internet;

    use crate::assembly::setup::Common;
    use crate::stores::account::Account;

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
        let mut account = common.create_account().await;
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
        let account = common.create_account().await;
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
        let account = common.create_account().await;
        assert!(common.db.accounts.delete(&account).await.is_ok());
        let result = common
            .db
            .accounts
            .search(&account, ComparisonOperator::Equal, ConditonalOperator::AND)
            .await;
        assert!(result.is_err())
    }
}
