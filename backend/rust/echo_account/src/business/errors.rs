use std::{error::Error, fmt};

use echo_rabbit::generic::RabbitError;
use echo_redis::generic::RedisError;
use echo_sql::generic::PostgresError;

#[derive(Debug)]
pub enum ServiceError {
    Redis(RedisError),
    Postgres(PostgresError),
    Rabbit(RabbitError),
    Generic(String),
}

impl Error for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::Redis(s) => write!(f, "{}", s),
            ServiceError::Postgres(s) => write!(f, "{}", s),
            ServiceError::Rabbit(s) => write!(f, "{}", s),
            ServiceError::Generic(s) => write!(f, "{}", s),
        }
    }
}
