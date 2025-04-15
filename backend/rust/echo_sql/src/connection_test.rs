#[cfg(test)]
mod tests {
    use crate::connection::Config;

    #[tokio::test]
    async fn test_connection() {
        let config = Config::new();
        let conn = config.connect().await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_migration() {
        let explicit_testing_flag = true;
        if explicit_testing_flag {
            println!("cargo:rerun-if-changed=migrations");
            let config = Config::new();
            let conn = config.connect().await.unwrap();
            let migrations = config.migrate(conn).await;
            assert!(migrations.is_ok());
        }
    }
}
