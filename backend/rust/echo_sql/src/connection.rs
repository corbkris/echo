use sqlx::migrate::MigrateError;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Error,
};
use std::env;

pub type PostgresPool = PgPool;

pub struct Config {
    name: String,
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl Config {
    pub fn new() -> Self {
        let name = env::var("POSTGRES_DB").unwrap();
        let host = env::var("POSTGRES_HOST").unwrap();
        let port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
        let user = env::var("POSTGRES_USER").unwrap();
        let password = env::var("POSTGRES_PASSWORD").unwrap();

        Self {
            name,
            host,
            port,
            user,
            password,
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }

    pub async fn connect(&self) -> Result<PostgresPool, Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.connection_string())
            .await
    }

    pub async fn migrate(&self, pool: PgPool) -> Result<(), MigrateError> {
        sqlx::migrate!("src/migrations/").run(&pool).await
    }
}

//private tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_string() {
        let connection_string = Config::new().connection_string();
        assert_eq!(
            connection_string,
            "postgresql://myuser:mypassword@localhost:5432/mydatabase"
        )
    }
}
