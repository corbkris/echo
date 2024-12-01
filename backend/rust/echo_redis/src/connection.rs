use redis::{Client, RedisError};
use std::env;
pub type RedisClient = Client;

pub struct Config {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: env::var("REDIS_HOST").unwrap(),
            port: env::var("REDIS_PORT").unwrap().parse::<u16>().unwrap(),
            user: env::var("REDIS_USER").unwrap(),
            password: env::var("REDIS_PASSWORD").unwrap(),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }

    pub fn connection_string_develop(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }

    pub fn connect(&self) -> Result<RedisClient, RedisError> {
        Client::open(self.connection_string_develop().as_str())
    }
}
