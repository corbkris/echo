#[cfg(test)]
mod tests {

    use crate::{assembly::setup::Common, queues::email::EmailSigup};
    #[tokio::test]
    async fn test_send_que_message() {
        let common = Common::new().await;
        let email = EmailSigup::new("email@gmail.com".to_string(), "12345".to_string());
        common
            .que
            .email_channel
            .tx_select()
            .await
            .expect("failed to start tx");
        let result = common
            .que
            .emails
            .publish_email(common.que.email_channel, &email)
            .await;
        common
            .que
            .email_channel
            .tx_commit()
            .await
            .expect("failed to commit transaction");
        assert!(result.is_ok());
    }
}
