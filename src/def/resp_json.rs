#[allow(unused)]

use crate::err::Result;

use std::future::Future;
use axum::{
    body, http, response::{ IntoResponse, Response }
};
use serde;
use serde_json;


#[derive(Debug, serde::Serialize)]
pub struct Payload<T: serde::Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}
impl<T: serde::Serialize> Payload<T> {
    pub fn data(data: T) -> Self {
        Payload {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
        }
    }
    pub fn empty() -> Self {
        Payload {
            code: 0,
            message: "ok".to_string(),
            data: None,
        }
    }
}

#[derive(Debug)]
pub struct ApplicationJson<T: serde::Serialize> {
    pub payload: T
}
impl<T: serde::Serialize> ApplicationJson<T> {
    pub fn new(data: T) -> Self {
        ApplicationJson { payload: data }
    }
}
impl<T: serde::Serialize> ApplicationJson<Payload<T>> {
    pub fn payload(data: T) -> Self {
        let payload = Payload::data(data);
        ApplicationJson { payload }
    }
    pub fn from_result(res: Result<T>) -> ApplicationJson<Payload<T>> {
        match res {
            Ok(data) => {
                let payload = Payload::data(data);
                ApplicationJson { payload }
            },
            Err(err) => {
                let mut payload = Payload::<T>::empty();
                payload.code = err.to_code();
                payload.message = err.to_message();
                ApplicationJson { payload }
            }
        }
    }
}
impl<T: serde::Serialize> IntoResponse for ApplicationJson<T> {
    fn into_response(self) -> Response<body::Body> {
        let json = serde_json::to_string(&self.payload).unwrap();
        let body = body::Body::new(json);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}

#[allow(unused)]
pub async fn application_json_handler<F, T: serde::Serialize>(future: F) -> Response
where  
    F: Future<Output = Result<T>>,  
{  
    let res = future.await;
    match res {
        Ok(data) => {
            let payload = Payload::data(data);
            ApplicationJson { payload }.into_response()
        },
        Err(err) => {
            let mut payload = Payload::<String>::empty();
            payload.code = err.to_code();
            payload.message = err.to_message();
            ApplicationJson { payload }.into_response()
        }
    }
}

