use hyper::{Body, Request, Response, StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    Unauthorized,
    Internal(String),
    BadRequest(String),
    NotFound(String),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Internal(s) => write!(f, "Internal: {}", s),
            ApiError::BadRequest(s) => write!(f, "Bad Request: {}", s),
            ApiError::NotFound(s) => write!(f, "Not Found: {}", s),
        }
    }
}

pub async fn error_handler(err: routerify::RouteError) -> Response<Body> {
    let api_err = err.downcast::<ApiError>().unwrap();

    match api_err.as_ref() {
        ApiError::Unauthorized => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap(),
        ApiError::Internal(s) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(s.to_string()))
            .unwrap(),
        ApiError::BadRequest(s) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(s.to_string()))
            .unwrap(),
        ApiError::NotFound(s) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(s.to_string()))
            .unwrap(),
    }
}

pub async fn handler_404(_: Request<Body>) -> Result<Response<Body>, ApiError> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page Not Found"))
        .unwrap())
}
