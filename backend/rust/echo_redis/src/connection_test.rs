#[cfg(test)]
mod tests {
    use crate::connection::{BasicClient, Config};

    #[test]
    fn it_works() {
        let config = Config::new();
        let client = BasicClient::new(config);
        assert!(client.is_ok())
    }
}
