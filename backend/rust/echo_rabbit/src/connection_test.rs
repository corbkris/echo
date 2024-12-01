#[cfg(test)]
mod tests {
    use crate::connection::Config;

    #[tokio::test]
    async fn test_connection() {
        let connection = Config::new().connect().await;

        assert!(connection.is_ok());
    }
}
