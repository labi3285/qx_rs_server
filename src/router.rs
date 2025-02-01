#[allow(unused)]

use axum::{
    response::IntoResponse,
    Router,
};

use crate::err::Error;


pub fn router() -> Router {
    let router: Router = Router::new().fallback(resp_404);
    router
}

async fn resp_404() -> impl IntoResponse {
    let err = Error::Paging(404, "page not found".to_string());
    err.into_response()
}