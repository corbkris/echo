use echo_rabbit::{
    generic::{Que, RabbitChannel, RabbitError},
    models::emails::EmailSignup as QueEmailSignup,
};
use serde_json;
pub type EmailSigup = QueEmailSignup;

const EXCHANGE: &str = "email_exchange";
const ROUTING_KEY: &str = "email_routing_key";
pub const EMAIL_QUE_NAME: &str = "email_que";

pub struct EmailQue<'a> {
    que: &'a Que<'a>,
    email_channel: &'a RabbitChannel,
}

impl<'a> EmailQue<'a> {
    pub fn new(que: &'a Que<'a>, email_channel: &'a RabbitChannel) -> Self {
        Self { que, email_channel }
    }

    pub async fn publish_email(&self, email: &EmailSigup) -> Result<(), RabbitError> {
        let payload = serde_json::to_string(email).unwrap();
        self.que
            .publish_message(self.email_channel, EXCHANGE, ROUTING_KEY, &payload)
            .await
    }
}
