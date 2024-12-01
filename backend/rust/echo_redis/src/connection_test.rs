#[cfg(test)]
mod tests {
    use crate::connection::Config;

    #[test]
    fn it_works() {
        let client = Config::new().connect();
        assert!(client.is_ok());
    }
}
