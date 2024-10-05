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

#[derive(Clone)]
pub struct EmailChannel {
    channel: RabbitChannel,
}

#[derive(Clone)]
pub struct EmailQue {
    que: Que,
    email_channel: EmailChannel,
}

impl EmailQue {
    pub async fn new(mut que: Que) -> Self {
        let email_channel = match que.create_channel(QUE_NAME).await {
            Ok(channel) => EmailChannel { channel },
            Err(err) => {
                panic!("{}", err);
            }
        };
        Self { que, email_channel }
    }

    pub async fn publish_email(&mut self, email: &EmailBody) -> Result<(), RabbitError> {
        let payload = serde_json::to_string(email).unwrap();
        self.que
            .publish_message(
                self.email_channel.channel.clone(),
                EXCHANGE,
                ROUTING_KEY,
                &payload,
            )
            .await
    }
}
