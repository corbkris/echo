use amqprs::{
    connection::{Connection, OpenConnectionArguments},
    error::Error,
};
use std::env;

pub type RabbitConnection = Connection;

pub struct Config {
    host: String,
    port: u16,
    user: String,
    password: String,
    vhost: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: env::var("RABBIT_HOST").unwrap(),
            port: env::var("RABBIT_PORT").unwrap().parse::<u16>().unwrap(),
            user: env::var("RABBIT_USER").unwrap(),
            password: env::var("RABBIT_PASSWORD").unwrap(),
            vhost: env::var("RABBIT_VHOST").unwrap(),
        }
    }
    pub async fn connect(&self) -> Result<RabbitConnection, Error> {
        Connection::open(
            &OpenConnectionArguments::new(&self.host, self.port, &self.user, &self.password)
                .virtual_host(&self.vhost),
        )
        .await
    }
    pub async fn close_connection(&self, connection: RabbitConnection) -> Result<(), Error> {
        connection.close().await
    }
}
