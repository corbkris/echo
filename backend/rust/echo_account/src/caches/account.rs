use echo_redis::{
    generic::{Cache, RedisError},
    models::account::Account as CacheAccount,
};
use serde_json;

const BASE_RATE_LIMIT: &str = "ratelimiter:";
const BASE_SIGNUP: &str = "signup:";
pub type Account = CacheAccount;

pub struct AccountCache<'a> {
    cache: &'a Cache<'a>,
}

impl<'a> AccountCache<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        Self { cache }
    }

    pub async fn set(&self, key: &str, value: &String) -> Result<String, RedisError> {
        self.cache
            .set(&[BASE_RATE_LIMIT, key].join("::"), value)
            .await
    }

    pub async fn get(&self, key: &str) -> Result<String, RedisError> {
        self.cache.get(&[BASE_RATE_LIMIT, key].join("::")).await
    }

    pub async fn set_signup(&self, key: &str, value: &Account) -> Result<String, RedisError> {
        self.cache
            .set_exp(
                &[BASE_SIGNUP, key].join("::"),
                &serde_json::to_string(value).unwrap(),
                120,
            )
            .await
    }

    pub async fn get_signup(&self, key: &str) -> Result<Account, String> {
        let result = match self.cache.get(&[BASE_SIGNUP, key].join("::")).await {
            Ok(result) => result,
            Err(err) => return Err(err.to_string()),
        };
        // Deserialize the JSON string into an Account instance
        match serde_json::from_str(&result) {
            Ok(account) => Ok(account),
            Err(err) => Err(err.to_string()),
        }
    }
}
