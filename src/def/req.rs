use crate::err::{Result, Error};
use axum::http;

pub fn get_http_header_val(headers: &http::HeaderMap, key: &str) -> Result<Option<String>> {
    let res: Option<&http::HeaderValue> = headers.get::<String>(key.to_string());
    if let Some(val) = res {
        let res = val.to_str();
        match res {
            Ok(e) => Ok(Some(e.to_string())),
            Err(err) => {
                tracing::error!("{}", err);
                Err(Error::ReadingHeaderFromRequest(key.to_string()))
            }
        }
    } else {
        Ok(None)
    }
}
