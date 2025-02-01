#[allow(unused)]


use axum::{
    body, http, response::{ IntoResponse, Response }
};


#[derive(Debug)]
pub struct TextPlain {
    pub payload: String
}
impl IntoResponse for TextPlain {
    fn into_response(self) -> Response<body::Body> {
        let body = body::Body::new(self.payload);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(body)
            .unwrap()
    }
}
