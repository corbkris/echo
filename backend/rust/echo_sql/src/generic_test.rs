#[cfg(test)]
mod tests {
    use crate::connection::Config;
    use crate::generic::DB;
    use crate::models::account::Account;

    #[tokio::test]
    async fn test_db() {
        let config = Config::new();
        let conn = match config.connect().await {
            Ok(conn) => conn,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let mut db = DB::new(conn);
        match db
            .insert(&Account::new(
                "".to_string(),
                "cardboard1234".to_string(),
                "corbin1234".to_string(),
                None,
                None,
            ))
            .await
        {
            Ok(account) => {
                assert_ne!(account.id, "")
            }
            Err(err) => {
                panic!("{}", err)
            }
        };
    }
}
