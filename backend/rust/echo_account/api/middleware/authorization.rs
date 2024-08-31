use echo_account::business::accounts::service::Service as account_service;
use echo_jwt::account::verify_account_token;
use hyper::{header::HeaderValue, Body, Request};
use routerify::prelude::*;
use serde_json::to_string;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;

pub struct Service {
    pub account_service: account_service,
}

impl Service {
    pub fn new(account_service: account_service) -> Self {
        Self { account_service }
    }
}

pub async fn full(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    // Clone the token if it exists.
    let token = match req.headers().get("account_token") {
        None => return Ok(req), // Early return if no token found.
        Some(token) => token.to_str().unwrap().to_owned(), // Clone the token string to avoid moving req.
    };

    // Extract the service data first.
    let service_arc = req.data::<Arc<Mutex<Service>>>().unwrap().clone(); // Clone the Arc pointer.

    // Unlock the state and modify it.
    let mut state = service_arc.lock().await;

    let account_id = verify_account_token(&token).unwrap(); // Use the cloned token.
    let account = state.account_service.find_by_id(&account_id).await.unwrap();
    let serialized_account = to_string(&account).unwrap();

    let headers = req.headers().clone();

    let mut new_req = Request::builder()
        .method(req.method())
        .uri(req.uri())
        .body(req.into_body())
        .expect("Failed to build request");

    // Insert the original headers into the new request
    *new_req.headers_mut() = headers;

    // Add the new header
    new_req.headers_mut().insert(
        "account_info",
        HeaderValue::from_str(&serialized_account).unwrap(),
    );

    // Return the new request
    Ok(new_req)
}
