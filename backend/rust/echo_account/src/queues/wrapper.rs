use echo_rabbit::generic::RabbitChannel;

use crate::queues::email::EmailQue;

pub struct EchoQue<'a> {
    pub emails: &'a EmailQue<'a>,
    pub email_channel: &'a RabbitChannel,
}

impl<'a> EchoQue<'a> {
    pub fn new(emails: &'a EmailQue<'a>, email_channel: &'a RabbitChannel) -> Self {
        Self {
            emails,
            email_channel,
        }
    }
}
