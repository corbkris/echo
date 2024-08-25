use crate::caches::account::Account;
use echo_redis::generic::Cache;
use redis::RedisError;
use serde_json;

const BASE_RATE_LIMIT: &str = "ratelimiter:";
const BASE_SIGNUP: &str = "signup:";

#[derive(Clone)]
pub struct AccountCache {
    cache: Cache,
}

impl AccountCache {
    pub fn new(cache: Cache) -> Self {
        Self { cache }
    }

    pub async fn set(&mut self, key: &str, value: String) -> Result<String, RedisError> {
        self.cache
            .set(vec![BASE_RATE_LIMIT, key].join("::"), value)
            .await
    }

    pub async fn set_signup(&mut self, key: &str, value: &Account) -> Result<String, RedisError> {
        self.cache
            .set_exp(
                vec![BASE_SIGNUP, key].join("::"),
                serde_json::to_string(value).unwrap(),
                120,
            )
            .await
    }

    pub async fn get_signup(&mut self, key: &str) -> Result<Account, String> {
        let result = match self.cache.get(vec![BASE_SIGNUP, key].join("::")).await {
            Ok(result) => result,
            Err(err) => return Err(err.to_string()),
        };
        // Deserialize the JSON string into an Account instance
        match serde_json::from_str(&result) {
            Ok(account) => Ok(account),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get(&mut self, key: &str) -> Result<String, RedisError> {
        self.cache.get(key.to_string()).await
    }
}
