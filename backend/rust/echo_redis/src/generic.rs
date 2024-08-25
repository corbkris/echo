use redis::AsyncCommands;
use redis::Client;
use redis::RedisError;

#[derive(Clone)]
pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&mut self, key: String) -> Result<String, RedisError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.get(key).await
    }

    pub async fn set(&mut self, key: String, value: String) -> Result<String, RedisError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.set(key, value).await
    }

    pub async fn incr(&mut self, key: String) -> Result<String, RedisError> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        conn.incr(key, 1).await
    }

    pub async fn set_exp(
        &mut self,
        key: String,
        value: String,
        exp: i64,
    ) -> Result<String, RedisError> {
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

    pub async fn incr_exp(&mut self, key: String, exp: i64) -> Result<String, RedisError> {
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
