#[cfg(test)]
mod tests {
    use crate::connection::{BasicConnection, Config};

    #[tokio::test]
    async fn test_connection() {
        let config = Config::new();
        let conn = BasicConnection::new(config).await;

        assert!(conn.is_ok());
    }
}
