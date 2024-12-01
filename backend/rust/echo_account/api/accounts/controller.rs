use crate::accounts::acceptor::marshal_signup;
use crate::middleware::error::ApiError;
use echo_account::business::accounts::service::Service as account_service;
use hyper::{Body, Request, Response};
use routerify::prelude::*;

pub struct AccountState<'a> {
    pub account_service: &'a account_service<'a>,
}

impl<'a> AccountState<'a> {
    pub fn new(account_service: &'a account_service) -> Self {
        AccountState { account_service }
    }
}

pub async fn signup(req: Request<Body>) -> Result<Response<Body>, ApiError> {
    let (parts, body) = req.into_parts();

    let signup_data = marshal_signup(body).await?;

    let state = parts.data::<AccountState>().unwrap();

    let secret_key = match state
        .account_service
        .signup(signup_data.email, signup_data.password)
        .await
    {
        Ok(secret_key) => secret_key,
        Err(_) => return Err(ApiError::Generic("Failed to signup user".into())),
    };

    let second_signup = state
        .account_service
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
        Err(_) => return Err(ApiError::Generic("Failed to signup user".into())),
    }

    Ok(Response::new(
        format!("Signup for user: {}", secret_key).into(),
    ))
}
