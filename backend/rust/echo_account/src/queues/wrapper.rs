use echo_rabbit::generic::{Que, RabbitChannel};

use crate::queues::email::EmailQue;

pub struct EchoQue<'a> {
    pub emails: Box<EmailQue<'a>>,
    pub email_channel: Box<RabbitChannel>,
}

impl<'a> EchoQue<'a> {
    pub async fn new(que: &'a Que<'a>) -> Self {
        let emails = Box::new(EmailQue::new(que));
        let email_channel = Box::new(emails.create_email_channel().await.unwrap());

        Self {
            emails,
            email_channel,
        }
    }
}
