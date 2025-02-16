use echo_rabbit::{
    generic::{Que, RabbitChannel, RabbitError},
    models::emails::EmailSignup as QueEmailSignup,
};
use serde_json;
pub type EmailSigup = QueEmailSignup;

pub const EMAIL_QUE_NAME: &str = "email_que";
const EMAIL_EXCHANGE: &str = "my_exchange";
const EMAIL_ROUTING_KEY: &str = "email_key";

pub struct EmailQue<'a> {
    que: &'a Que<'a>,
}

impl<'a> EmailQue<'a> {
    pub fn new(que: &'a Que<'a>) -> Self {
        Self { que }
    }

    pub async fn create_email_channel(&self) -> Result<RabbitChannel, RabbitError> {
        self.que
            .create_channel(EMAIL_QUE_NAME, EMAIL_EXCHANGE, EMAIL_ROUTING_KEY)
            .await
    }

    pub async fn publish_email(
        &self,
        channel: &RabbitChannel,
        email: &EmailSigup,
    ) -> Result<(), RabbitError> {
        let payload = serde_json::to_string(email).unwrap();
        self.que
            .publish_message(channel, EMAIL_EXCHANGE, EMAIL_ROUTING_KEY, &payload)
            .await
    }
}
