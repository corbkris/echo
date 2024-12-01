use crate::queues::email::EmailQue;

pub struct EchoQue<'a> {
    pub emails: &'a EmailQue<'a>,
}

impl<'a> EchoQue<'a> {
    pub fn new(emails: &'a EmailQue<'a>) -> Self {
        Self { emails }
    }
}
