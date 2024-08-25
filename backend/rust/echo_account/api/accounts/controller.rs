use echo_account::business::{account::Account, accounts::service::Service as account_service};
use hyper::{Body, Request, Response};
use routerify::prelude::*;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Service {
    pub account: Option<Account>,
    pub account_service: account_service,
}

impl Service {
    pub fn new(account_service: account_service) -> Self {
        Service {
            account_service,
            account: None,
        }
    }
}
pub async fn signup(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let service = req.data::<Arc<Mutex<Service>>>().unwrap();
    let mut service = service.lock().await; // Lock to get mutable access (await async Mutex)

    let _ = service.account_service.try_signup_code("", "").await;

    Ok(Response::new(format!("{} db connections", 1).into()))
}
