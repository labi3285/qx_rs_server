use axum::{body, http, response::{IntoResponse, Response}};

use crate::err::Error;
use crate::def::resp_json::Payload;


impl Error {
    pub fn to_code(&self) -> i32 {
        match self {
            Error::Env(_) => -1,
            Error::ServerStart(_) => -2,
            Error::ReadingHeaderFromRequest(_) => -2,
            Error::Parsing(_) => -100,
            Error::ThirdRequest(_) => -200,
            Error::Paging(code, _) => *code,
            Error::Custom(code, _) => *code,
            Error::Io(_) => -1,
        }
    }
    pub fn to_message(&self) -> String {
        match self {
            Error::Env(err) => err.clone(),
            Error::ServerStart(err) => err.clone(),
            Error::ReadingHeaderFromRequest(err) => err.clone(),
            Error::Parsing(err) => err.clone(),
            Error::ThirdRequest(err) => err.clone(),
            Error::Paging(_, msg) => msg.clone(),
            Error::Custom(_, msg) => msg.clone(),
            Error::Io(_) => "i/o error".to_string(),
        }
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
