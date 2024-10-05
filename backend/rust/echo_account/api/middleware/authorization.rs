use echo_account::business::account::Account;
use echo_account::business::accounts::service::Service as account_service;
use echo_jwt::account::get_account_id_from_token;
use hyper::{header::HeaderValue, Body, Request};
use routerify::{prelude::*, Error};
use serde_json::{from_str, to_string};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Service {
    pub account_service: Arc<Mutex<account_service>>,
}

impl Service {
    pub fn new(account_service: Arc<Mutex<account_service>>) -> Self {
        Self { account_service }
    }
}

pub async fn full(req: Request<Body>) -> Result<Request<Body>, Error> {
    let token = match req.headers().get("account_token") {
        Some(token) => token.to_str().unwrap().to_owned(),
        None => return Err(Error::new("Nil token")),
    };

    let (parts, body) = req.into_parts();

    let state = parts.data::<Arc<Mutex<Service>>>().unwrap().lock().await; // Clone the Arc pointer.

    let account_id = get_account_id_from_token(&token).unwrap(); // Use the cloned token.
    let account = state
        .account_service
        .lock()
        .await
        .find_by_id(&account_id)
        .await
        .unwrap();
    let serialized_account = to_string(&account).unwrap();

    let mut new_req = Request::builder()
        .method(parts.method.clone())
        .uri(parts.uri.clone())
        .body(body)
        .expect("Failed to build request");

    // Insert the original headers into the new request
    *new_req.headers_mut() = parts.headers.clone();

    // Add the new header
    new_req.headers_mut().insert(
        "account_info",
        HeaderValue::from_str(&serialized_account).unwrap(),
    );

    // Return the new request
    Ok(new_req)
}

pub async fn get_account_from_header(req: &Request<Body>) -> Result<Account, String> {
    // Attempt to get the `account_info` header
    let account_header = match req.headers().get("account_info") {
        Some(account_header) => account_header,
        None => return Err("Account header not found".to_string()),
    };

    // Convert the header value to a string
    let account_str = match account_header.to_str() {
        Ok(account_str) => account_str,
        Err(err) => return Err(err.to_string()),
    };

    // Deserialize the string into an Account struct
    match from_str::<Account>(account_str) {
        Ok(account) => Ok(account),
        Err(err) => Err(err.to_string()),
    }
}
