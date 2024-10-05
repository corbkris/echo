use redis::{Client, RedisError};
use std::env;

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
}

pub struct BasicClient {
    pub client: Client,
}

impl BasicClient {
    pub fn new(config: Config) -> Result<Self, RedisError> {
        match Client::open(config.connection_string_develop().as_str()) {
            Ok(client) => return Ok(Self { client }),
            Err(err) => return Err(err),
        };
    }
}