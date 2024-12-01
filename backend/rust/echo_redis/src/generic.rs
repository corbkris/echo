use redis::AsyncCommands;
use redis::RedisError;

use crate::connection::RedisClient;

pub type CacheError = RedisError;

pub struct Cache<'a> {
    client: &'a RedisClient,
}

impl<'a> Cache<'a> {
    pub fn new(client: &'a RedisClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, key: &String) -> Result<String, CacheError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.get(key).await
    }

    pub async fn set(&self, key: &String, value: &String) -> Result<String, CacheError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.set(key, value).await
    }

    pub async fn incr(&self, key: &String) -> Result<String, CacheError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.incr(key, 1).await
    }

    pub async fn set_exp(
        &self,
        key: &String,
        value: &String,
        exp: i64,
    ) -> Result<String, CacheError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        let resp: String = match conn.set(&key, value).await {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        match conn.expire(&key, exp).await {
            Ok(()) => Ok(resp),
            Err(err) => Err(err),
        }
    }

    pub async fn incr_exp(&self, key: &String, exp: i64) -> Result<String, CacheError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };

        let resp: String = match conn.incr(&key, 1).await {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        match conn.expire(&key, exp).await {
            Ok(()) => Ok(resp),
            Err(err) => Err(err),
        }
    }
}
