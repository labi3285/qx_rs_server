use axum::{body, http, response::{IntoResponse, Response}};

use crate::err::Error;
use crate::def::resp_json::Payload;


impl Error {
    pub fn to_code(&self) -> i32 {
        let mut code: i32 = -1;

        code
    }
    pub fn to_message(&self) -> String {
        "".to_string()
    }
}

impl Error {
    pub fn to_payload(&self) -> Payload<String> {
        Payload {
            code: self.to_code(),
            message: self.to_message(),
            data: None,
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response<body::Body> {
        let payload = self.to_payload();
        let json = serde_json::to_string(&payload).unwrap();
        let body = body::Body::new(json);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}
