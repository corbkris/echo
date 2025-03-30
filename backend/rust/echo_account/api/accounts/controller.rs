use crate::accounts::{
    acceptor::marshal_signup,
    presenter::{SignupPresenter, SignupResponse},
};
use crate::middleware::error::ApiError;
use echo_account::business::{accounts::service::Service as account_service, errors::ServiceError};
use echo_sql::generic::PostgresError;
use hyper::{header, Body, Request, Response, StatusCode};
use routerify::prelude::*;
use tracing::{error, instrument};

pub struct AccountState<'a> {
    pub account_service: &'a account_service<'a>,
}

impl<'a> AccountState<'a> {
    pub fn new(account_service: &'a account_service) -> Self {
        Self { account_service }
    }
}

#[instrument]
pub async fn basic_signup<'a>(req: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    let (parts, body) = req.into_parts();

    let state = parts.data::<AccountState>().unwrap();
    let signup_data = marshal_signup(body).await?;

    let username: &str = &signup_data.signup.username;
    let password: &str = &signup_data.signup.password;

    match state
        .account_service
        .find_account_by_username(username)
        .await
    {
        Ok(_) => {
            return Err(ApiError::BadRequest("username taken"));
        }
        Err(err) => {
            if !matches!(err, ServiceError::Postgres(PostgresError::RowNotFound)) {
                return Err(ApiError::Internal("failed to signup user"));
            }
        }
    }

    let recovery_key = match state.account_service.basic_signup(username, password).await {
        Ok(recovery_key) => recovery_key,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("failed to signup user"));
        }
    };

    let body = match serde_json::to_string(&SignupPresenter {
        signup: SignupResponse {
            username: username.to_string(),
            recovery_key,
        },
    }) {
        Ok(json) => json,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("failed to signup user"));
        }
    };

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap())
}
