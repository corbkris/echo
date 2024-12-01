#[cfg(test)]
mod tests {

    use crate::queues::{email::EmailSigup, testing::test::Common};
    #[tokio::test]
    async fn test_set() {
        let common = Common::new().await;
        let email = EmailSigup::new("email".to_string(), "code".to_string());
        let result = common.que.emails.publish_email(&email).await;
        assert!(result.is_ok());
    }
}
