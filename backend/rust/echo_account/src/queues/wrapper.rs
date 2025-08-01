use echo_rabbit::{connection::RabbitConnection, generic::RabbitChannel};

use crate::queues::email::EmailQue;

pub struct EchoQue<'a> {
    pub emails: EmailQue<'a>,
    pub email_channel: RabbitChannel,
}

impl<'a> EchoQue<'a> {
    pub async fn new(connection: &'a RabbitConnection) -> Self {
        let emails = EmailQue::new(connection);
        let email_channel = emails.create_email_channel().await.unwrap();

        Self {
            emails,
            email_channel,
        }
    }
}
