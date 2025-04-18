#[cfg(test)]
mod tests {
    use crate::connection::Config;
    use crate::generic::DB;
    use crate::tables::account::Account;

    #[tokio::test]
    async fn test_db() {
        let config = Config::new();
        let conn = match config.connect().await {
            Ok(conn) => conn,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let db = DB::new(&conn);
        let mut account = Account::new(None, "cardboard1234".to_string(), None, None);
        let result = db.insert(&mut account).await;
        assert!(result.is_none());
        assert_ne!(account.id, None);
    }
}
