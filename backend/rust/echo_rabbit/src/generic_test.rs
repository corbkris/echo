#[cfg(test)]
mod tests {
    use crate::connection::{BasicConnection, Config};
    use crate::generic::Que;

    #[tokio::test]
    async fn test_generic() {
        let config = Config::new();
        let connection = match BasicConnection::new(config).await {
            Ok(conn) => conn.connection,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let mut que = Que::new(connection);
        let channel_name = "my_channel";
        let channel = match que.create_channel(channel_name).await {
            Ok(channel) => channel,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let (exchange, routing_key, payload): (&str, &str, &str) =
            ("my_exchange", "my_routing_key", "my_payload");
        let result = que
            .publish_message(channel, exchange, routing_key, payload)
            .await;
        assert!(result.is_ok());
    }
}
