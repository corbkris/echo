use echo_account::business::{account::Account, accounts::service::Service as account_service};
use echo_jwt::account::verify_account_token;
use hyper::{Body, Request};
use routerify::prelude::*;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;

pub struct State {
    account_service: account_service,
    account: Account,
}

pub async fn full(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    // Clone the token if it exists.
    let token = match req.headers().get("account_token") {
        None => return Ok(req), // Early return if no token found.
        Some(token) => token.to_str().unwrap().to_owned(), // Clone the token string to avoid moving req.
    };

    // Extract the service data first.
    let service_arc = req.data::<Arc<Mutex<State>>>().unwrap().clone(); // Clone the Arc pointer.

    // Unlock the state and modify it.
    let mut state = service_arc.lock().await;

    let account_id = verify_account_token(&token).unwrap(); // Use the cloned token.
    let account = state.account_service.find_by_id(&account_id).await.unwrap();
    state.account = account;

    // Finally, return the request.
    Ok(req)
}
