use hyper::{Body, Request, Response, StatusCode};
use routerify::{prelude::*, Error, RouteError};

pub async fn logger_handler(req: Request<Body>) -> Result<Request<Body>, Error> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

pub async fn handler_error(err: RouteError) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(err.to_string()))
        .unwrap()
}

pub async fn handler_404(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page Not Found"))
        .unwrap())
}
