#[cfg(test)]
mod tests {
    use crate::caches::testing::test::Common;
    #[tokio::test]
    async fn test_set() {
        let common = Common::new().await;
        let key = "account_id".to_string();
        let value = "1234".to_string();
        let result = common.cache.accounts.set(&key, &value).await;
        assert!(result.is_ok());

        let result = common.cache.accounts.get(&key).await;
        assert!(result.is_ok());
        assert_eq!(&value, &result.unwrap())
    }
}
