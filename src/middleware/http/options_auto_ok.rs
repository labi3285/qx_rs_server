#[allow(unused)]


use axum::{
    body,
    extract::Request,
    http,
    middleware::Next,
    response::Response
};

pub async fn middleware_fn(
    request: Request,
    next: Next,
) -> Response {
    let body = body::Body::new("ok".to_string());
    match *request.method() {
        http::Method::OPTIONS => {
            let resp = Response::builder()
            .status(http::StatusCode::OK)
            .body(body)
            .unwrap();
            resp
        },
        _ => {
            let resp = next.run(request).await;
            resp
        }
    }
}

