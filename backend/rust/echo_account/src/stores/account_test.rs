#[cfg(test)]
mod tests {
    use crate::assembly::setup::Common;
    use echo_sql::basic::{ComparisonOperator, ConditonalOperator};
    use echo_sql::models::account::Account;

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
        let account = accounts
            .insert(Account::new(
                "".to_string(),
                "corbin12345".to_string(),
                "mypass1".to_string(),
                Some(7),
                Some(true),
                None,
                None,
            ))
            .await;
        assert!(account.is_ok());
        assert_ne!(account.unwrap().id, "");
    }

    #[tokio::test]
    async fn test_user_insert_nil() {
        let common = test_setup_common().await;

        let mut accounts = common.db.accounts;
        let account = accounts
            .insert(Account::new(
                "".to_string(),
                "corbin2680".to_string(),
                "mypass1".to_string(),
                None,
                None,
                None,
                None,
            ))
            .await;
        assert!(account.is_ok());
        assert_ne!(account.unwrap().id, "");
    }

    #[tokio::test]
    async fn test_user_stores_basic() {
        let common = test_setup_common().await;

        let mut accounts = common.db.accounts;
        let expected_account_result = accounts
            .insert(Account::new(
                "".to_string(),
                "corbin12345".to_string(),
                "mypass1".to_string(),
                Some(7),
                Some(true),
                None,
                None,
            ))
            .await;
        assert!(expected_account_result.is_ok());
        let expected_account = expected_account_result.unwrap();

        let actual_account_result = accounts
            .basic_search_single(
                Account::new(
                    expected_account.id.clone(),
                    "".to_string(),
                    "".to_string(),
                    None,
                    None,
                    None,
                    None,
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            )
            .await;
        assert!(actual_account_result.is_ok());

        let actual_account = actual_account_result.unwrap();
        assert_eq!(expected_account.id, actual_account.id);

        let updated_account_result = accounts
            .update(Account::new(
                actual_account.id.clone(),
                "corbin2680".to_string(),
                "".to_string(),
                None,
                None,
                None,
                None,
            ))
            .await;
        assert!(updated_account_result.is_ok());
        let updated_account = updated_account_result.unwrap();

        let actual_account_result = accounts
            .basic_search_single(
                Account::new(
                    actual_account.id.clone(),
                    "".to_string(),
                    "".to_string(),
                    None,
                    None,
                    None,
                    None,
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            )
            .await;
        assert!(actual_account_result.is_ok());
        let actual_account = actual_account_result.unwrap();
        assert_eq!(actual_account.email, updated_account.email);

        let deleted_account_result = accounts
            .delete(Account::new(
                actual_account.id.clone(),
                "".to_string(),
                "".to_string(),
                None,
                None,
                None,
                None,
            ))
            .await;
        assert!(deleted_account_result.is_ok());

        let actual_account_result = accounts
            .basic_search_single(
                Account::new(
                    actual_account.id.clone(),
                    "".to_string(),
                    "".to_string(),
                    None,
                    None,
                    None,
                    None,
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            )
            .await;
        assert!(!actual_account_result.is_ok())
    }
}
