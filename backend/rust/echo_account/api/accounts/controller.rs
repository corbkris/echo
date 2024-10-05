use crate::accounts::acceptor::marshal_signup;
use echo_account::business::accounts::service::Service as account_service;
use hyper::{Body, Request, Response};
use routerify::{prelude::*, Error};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct State {
    pub account_service: Arc<Mutex<account_service>>,
}

impl State {
    pub fn new(account_service: Arc<Mutex<account_service>>) -> Self {
        State { account_service }
    }
}

pub async fn signup(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (parts, body) = req.into_parts();

    let signup_data = marshal_signup(body).await?;

    let state = parts.data::<Arc<Mutex<State>>>().unwrap().lock().await;

    let secret_key = match state
        .account_service
        .lock()
        .await
        .signup(signup_data.email, signup_data.password)
        .await
    {
        Ok(secret_key) => secret_key,
        Err(_) => return Err(Error::new("Failed to sign up")),
    };

    let second_signup = state
        .account_service
        .lock()
        .await // Locking the account service again
        .signup(
            "another_email@example.com".to_string(),
            "another_password".to_string(),
        ) // Change as needed
        .await;

    match second_signup {
        Ok(secret_key) => {
            println!("Second signup successful, secret key: {}", secret_key);
            // Do something with the second signup result
        }
        Err(_) => return Err(Error::new("Failed to sign up second user")),
    }

    Ok(Response::new(
        format!("Signup for user: {}", secret_key).into(),
    ))
}
