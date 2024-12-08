#[cfg(test)]
mod tests {
    use crate::assembly::setup::Common;

    #[tokio::test]
    async fn test_set() {
        let common = Common::new().await;
        let result = common
            .services
            .account_service
            .signup("email".to_string(), "password".to_string())
            .await;
        assert!(result.is_ok());

        let result = common
            .services
            .account_service
            .signup("email".to_string(), "password".to_string())
            .await;
        assert!(result.is_ok());
    }
}
