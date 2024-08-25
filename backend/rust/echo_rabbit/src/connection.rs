use amqprs::{
    connection::{Connection, OpenConnectionArguments},
    error::Error,
};
use std::env;

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
}

pub struct BasicConnection {
    pub connection: Connection,
}

impl BasicConnection {
    pub async fn new(config: Config) -> Result<Self, Error> {
        match Connection::open(
            &OpenConnectionArguments::new(
                &config.host,
                config.port,
                &config.user,
                &config.password,
            )
            .virtual_host(&config.vhost),
        )
        .await
        {
            Ok(connection) => Ok(Self { connection }),
            Err(err) => Err(err),
        }
    }
    pub async fn close_connection(self) -> Result<(), Error> {
        self.connection.close().await
    }
}
