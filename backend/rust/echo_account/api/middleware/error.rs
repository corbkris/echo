use hyper::{Body, Response, StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    Unauthorized,
    Generic(String),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Generic(s) => write!(f, "Generic: {}", s),
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
        ApiError::Generic(s) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(s.to_string()))
            .unwrap(),
    }
}
