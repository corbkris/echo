use crate::middleware::error::ApiError;
use echo_account::business::account::Account;
use echo_account::business::accounts::service::Service as account_service;
use echo_jwt::account::get_account_id_from_token;
use hyper::{Body, Request};
use routerify::prelude::*;
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

pub async fn full_v2(req: Request<Body>) -> Result<Account, ApiError> {
    let token = match req.headers().get("account_token") {
        Some(token) => token.to_str().unwrap().to_owned(),
        None => return Err(ApiError::BadRequest("Nil token".into())),
    };

    let state = req.data::<Arc<Mutex<Service>>>().unwrap().lock().await; // Clone the Arc pointer.

    let account_id = get_account_id_from_token(&token).unwrap(); // Use the cloned token.
    let account = match state
        .account_service
        .lock()
        .await
        .find_by_id(&account_id)
        .await
    {
        Ok(account) => account,
        Err(_) => return Err(ApiError::Generic("failed to find account by token".into())),
    };
    Ok(account)
}
