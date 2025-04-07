use std::str::FromStr;

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
use uuid::Uuid;

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
                error!("{}", err);
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
            recovery_key: recovery_key.to_string(),
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

#[instrument]
pub async fn managed_signup<'a>(req: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    let (parts, _) = req.into_parts();

    let state = parts.data::<AccountState>().unwrap();
    let code = match parts.param("code") {
        Some(code) => code,
        None => {
            return Err(ApiError::BadRequest("missing code"));
        }
    };
    let req_id = match Uuid::from_str(
        parts
            .headers
            .get("x-signup-req-id")
            .unwrap()
            .to_str()
            .unwrap(),
    ) {
        Ok(req_id) => req_id,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::BadRequest("invalid/missing req_id"));
        }
    };

    if let Some(err) = state.account_service.managed_signup(req_id, code).await {
        error!("{}", err);
        return Err(ApiError::Internal("failed to signup user"));
    };

    if let Some(err) = state
        .account_service
        .delete_signup_verification_by_req_id(req_id)
        .await
    {
        error!("{}", err);
    };

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

#[instrument]
pub async fn send_managed_signup_code<'a>(
    req: Request<Body>,
) -> Result<Response<Body>, ApiError<'a>> {
    let (parts, body) = req.into_parts();

    let state = parts.data::<AccountState>().unwrap();
    let signup_data = marshal_signup(body).await?;

    let email: &str = &signup_data.signup.email.unwrap();
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
                error!("{}", err);
                return Err(ApiError::Internal("failed to signup user"));
            }
        }
    };

    match state.account_service.find_account_by_email(email).await {
        Ok(_) => {
            return Err(ApiError::BadRequest("email taken"));
        }
        Err(err) => {
            if !matches!(err, ServiceError::Postgres(PostgresError::RowNotFound)) {
                error!("{}", err);
                return Err(ApiError::Internal("failed to signup user"));
            }
        }
    };

    let req_id = match state
        .account_service
        .send_managed_signup_verification_code(email, username, password)
        .await
    {
        Ok(req_id) => req_id,
        Err(err) => {
            error!("{}", err);
            return Err(ApiError::Internal("failed to signup user"));
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("x-signup-req-id", req_id.to_string())
        .body(Body::empty())
        .unwrap())
}
