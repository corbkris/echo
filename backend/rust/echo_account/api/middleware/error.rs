use hyper::{Body, Request, Response, StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum ApiError<'a> {
    #[allow(dead_code)]
    Unauthorized,
    Internal(&'a str),
    BadRequest(&'a str),
    BadRequestFmt(&'a str, &'a str),
    NotFound(&'a str),
}

impl<'a> std::error::Error for ApiError<'a> {}

impl<'a> fmt::Display for ApiError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Internal(s) => write!(f, "Internal: {}", s),
            ApiError::BadRequest(s) => write!(f, "Bad Request: {}", s),
            ApiError::BadRequestFmt(s, o) => write!(f, "Bad Request: {}: {}", s, o),
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

        ApiError::BadRequestFmt(s, o) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("{}: {}", s, o)))
            .unwrap(),
        ApiError::NotFound(s) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(s.to_string()))
            .unwrap(),
    }
}

pub async fn handler_404<'a>(_: Request<Body>) -> Result<Response<Body>, ApiError<'a>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page Not Found"))
        .unwrap())
}
