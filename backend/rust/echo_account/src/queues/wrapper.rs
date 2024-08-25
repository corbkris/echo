use crate::queues::email::EmailQue;
use echo_rabbit::generic::Que;

#[derive(Clone)]
pub struct EchoQue {
    pub emails: EmailQue,
}

impl EchoQue {
    pub async fn new(que: Que) -> Self {
        Self {
            emails: EmailQue::new(que), //handle error eventually
        }
    }
}
