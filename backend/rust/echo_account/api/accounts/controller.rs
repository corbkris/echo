use crate::accounts::{
    acceptors::{
        login::marshal_login,
        signup::{marshal_basic_signup, marshal_managed_signup},
    },
    presenter::{SignupPresenter, SignupResponse},
};
use crate::{
    middleware::error::ApiError,
    utility::{header::parse_header_uuid, route::parse_route_param},
};
use echo_account::business::{
    account::Account, accounts::service::Service as account_service, errors::ServiceError,
};
use echo_jwt::account::generate_auth_token;
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
pub async fn login<'a>(req: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    let (parts, body) = req.into_parts();

    let state = parts.data::<AccountState>().unwrap();
    let login_data = marshal_login(body).await?;

    let username = &login_data.login.username;
    let email = &login_data.login.email;
    let password = &login_data.login.password;

    let account = &mut Account::default();

    if let Some(username) = username {
        *account = match state
            .account_service
            .find_account_by_username_password(username, password)
            .await
        {
            Ok(account) => account,
            Err(err) => {
                if matches!(err, ServiceError::Postgres(PostgresError::RowNotFound)) {
                    error!("failed to find account by email, username: {}", err);
                    return Err(ApiError::NotFound("account not found"));
                } else {
                    error!("failed to search for account: {}", err);
                    return Err(ApiError::Internal("failed to search for account"));
                }
            }
        };
    };

    if let Some(email) = email {
        *account = match state
            .account_service
            .find_account_by_email_password(email, password)
            .await
        {
            Ok(account) => account,
            Err(err) => {
                if matches!(err, ServiceError::Postgres(PostgresError::RowNotFound)) {
                    error!("failed to find account by email, password: {}", err);
                    return Err(ApiError::NotFound("account not found"));
                } else {
                    error!("failed to search for account: {}", err);
                    return Err(ApiError::Internal("failed to search for account"));
                }
            }
        };
    };

    let token = match generate_auth_token(&account.id.unwrap().to_string()) {
        Ok(token) => token,
        Err(err) => {
            error!("failed to generate token: {}", err);
            return Err(ApiError::Internal("failed to login account"));
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("x-auth-token", token)
        .body(Body::empty())
        .unwrap())
}

#[instrument]
pub async fn basic_signup<'a>(req: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    let (parts, body) = req.into_parts();

    let state = parts.data::<AccountState>().unwrap();
    let signup_data = marshal_basic_signup(body).await?;

    let username = &signup_data.signup.username;
    let password = &signup_data.signup.password;

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
                error!("failed to search for account: {}", err);
                return Err(ApiError::Internal("failed to signup account"));
            }
        }
    }

    let recovery_key = match state.account_service.basic_signup(username, password).await {
        Ok(recovery_key) => recovery_key,
        Err(err) => {
            error!("failed to signup basic account: {}", err);
            return Err(ApiError::Internal("failed to signup account"));
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
            error!("failed to marshal signup response: {}", err);
            return Err(ApiError::Internal("failed to signup account"));
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
    let code = parse_route_param(parts.params(), "code")?;
    let req_id = parse_header_uuid(&parts.headers, "x-signup-req-id")?;

    if let Some(err) = state.account_service.managed_signup(req_id, &code).await {
        error!("failed to sign_up managed account: {}", err);
        return Err(ApiError::Internal("failed to signup account"));
    };

    if let Some(err) = state
        .account_service
        .delete_signup_verification_by_req_id(req_id)
        .await
    {
        error!("failed to delete signup_verification: {}", err);
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
    let signup_data = marshal_managed_signup(body).await?;

    let username = &signup_data.signup.username;
    let email = &signup_data.signup.email;
    let password = &signup_data.signup.password;

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
                error!("failed to search for account: {}", err);
                return Err(ApiError::Internal("failed to signup account"));
            }
        }
    };

    match state.account_service.find_account_by_email(email).await {
        Ok(_) => {
            return Err(ApiError::BadRequest("email taken"));
        }
        Err(err) => {
            if !matches!(err, ServiceError::Postgres(PostgresError::RowNotFound)) {
                error!("failed to search for account: {}", err);
                return Err(ApiError::Internal("failed to signup account"));
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
            error!("failed to send signup verification: {}", err);
            return Err(ApiError::Internal("failed to signup account"));
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("x-signup-req-id", req_id.to_string())
        .body(Body::empty())
        .unwrap())
}
