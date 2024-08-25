use echo_rabbit::generic::{Que, RabbitChannel, RabbitError};
use serde::Serialize;
use serde_json;

const EXCHANGE: &str = "email_exchange";
const ROUTING_KEY: &str = "email_routing_key";
const QUE_NAME: &str = "email_que";

#[derive(Serialize)]
pub struct EmailBody {
    email: String,
    code: String,
}

impl EmailBody {
    pub fn new(email: String, code: String) -> Self {
        Self { email, code }
    }
}
pub struct EmailChannel {
    channel: RabbitChannel,
}

#[derive(Clone)]
pub struct EmailQue {
    que: Que,
}

impl EmailQue {
    pub fn new(que: Que) -> Self {
        Self { que }
    }

    pub async fn create_channel(&mut self) -> Result<EmailChannel, RabbitError> {
        match self.que.create_channel(QUE_NAME).await {
            Ok(channel) => Ok(EmailChannel { channel }),
            Err(err) => Err(err),
        }
    }

    pub async fn publish_email(
        &mut self,
        email_channel: EmailChannel,
        email: &EmailBody,
    ) -> Result<(), RabbitError> {
        let payload = serde_json::to_string(email).unwrap();
        self.que
            .publish_message(email_channel.channel, EXCHANGE, ROUTING_KEY, &payload)
            .await
    }
}
