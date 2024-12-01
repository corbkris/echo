use async_once_cell::OnceCell;
use echo_rabbit::{
    connection::{Config, RabbitConnection},
    generic::{Que, RabbitChannel},
};

use crate::queues::{
    email::{EmailQue, EMAIL_QUE_NAME},
    wrapper::EchoQue,
};

pub static QUE_QUEUES: OnceCell<EchoQue> = OnceCell::new();
pub static QUE_CONNECTION: OnceCell<RabbitConnection> = OnceCell::new();
pub static QUE_QUEUE: OnceCell<Que> = OnceCell::new();
pub static QUE_EMAIL_CHANNEL: OnceCell<RabbitChannel> = OnceCell::new();
pub static QUE_EMAIL_QUEUE: OnceCell<EmailQue> = OnceCell::new();

pub struct Common<'a> {
    pub que: &'a EchoQue<'a>,
}
impl<'a> Common<'a> {
    pub async fn new() -> Self {
        setup().await;
        let que = QUE_QUEUES.get().unwrap();
        Self { que }
    }
}

async fn get_connection() -> RabbitConnection {
    Config::new().connect().await.unwrap()
}

async fn set_connection() {
    QUE_CONNECTION.get_or_init(get_connection()).await;
}

async fn get_que<'a>() -> Que<'a> {
    Que::new(QUE_CONNECTION.get().unwrap())
}

async fn set_que() {
    QUE_QUEUE.get_or_init(get_que()).await;
}

async fn get_email_channel() -> RabbitChannel {
    QUE_QUEUE
        .get()
        .unwrap()
        .create_channel(EMAIL_QUE_NAME)
        .await
        .unwrap()
}

async fn set_email_channel() {
    QUE_EMAIL_CHANNEL.get_or_init(get_email_channel()).await;
}

async fn get_email_que<'a>() -> EmailQue<'a> {
    EmailQue::new(QUE_QUEUE.get().unwrap(), QUE_EMAIL_CHANNEL.get().unwrap())
}

async fn set_email_que() {
    QUE_EMAIL_QUEUE.get_or_init(get_email_que()).await;
}

async fn get_echo_que<'a>() -> EchoQue<'a> {
    EchoQue::new(QUE_EMAIL_QUEUE.get().unwrap())
}

async fn set_echo_que() {
    QUE_QUEUES.get_or_init(get_echo_que()).await;
}

pub async fn setup() {
    set_connection().await;
    set_que().await;
    set_email_channel().await;
    set_email_que().await;
    set_echo_que().await;
}
