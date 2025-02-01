use reqwest::Client;
use reqwest::Response;
use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::err::{Error, Result};


pub async fn get_for_raw_text(url: &String, headers: &Option<Vec<(&str, &String)>>) -> Result<String> {
    let resp = get(url, headers).await?;
    let text = resp.text().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("parse failed".to_string())
    })?;
    Ok(text)
}

pub async fn get_for_object<O: DeserializeOwned>(url: &String, headers: &Option<Vec<(&str, &String)>>) -> Result<O> {
    let resp = get(url, headers).await?;
    let obj = resp.json::<O>().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("parse failed".to_string())
    })?;
    Ok(obj)
}

pub async fn post_application_json_for_raw_text<T: serde::Serialize>(url: &String, headers: &Option<Vec<(&str, &String)>>, data: &T) -> Result<String> {
    let payload = serde_json::json!(data).to_string();
    let resp = post_application_json(url, headers, &payload).await?;
    let text = resp.text().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("parse failed".to_string())
    })?;
    Ok(text)
}

pub async fn post_application_json_for_object<T: serde::Serialize, O: DeserializeOwned>(url: &String, headers: &Option<Vec<(&str, &String)>>, data: &T) -> Result<O> {
    let payload = serde_json::json!(data).to_string();
    let resp = post_application_json(url, headers, &payload).await?;    
    let obj = resp.json::<O>().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("parse failed".to_string())
    })?;
    Ok(obj)
}


pub async fn get(url: &String, headers: &Option<Vec<(&str, &String)>>) -> Result<Response> {
    let client = Client::new();
    let url = Url::parse(url).map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("parse failed".to_string())
    })?;
    let mut req = client.get(url);
    if let Some(headers) = headers {
        for header in headers {
            req = req.header(header.0, header.1.as_str());
        }
    }
    let resp = req.send().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest(format!("request failed: {:?}", err))
    })?;
    if resp.status().is_success() {
        Ok(resp)
    } else {
        Err(Error::ThirdRequest(format!("request failed ({:?})", resp.status())))
    }
}

pub async fn post_application_json(url: &String, headers: &Option<Vec<(&str, &String)>>, payload: &String) -> Result<Response> {
    let client = Client::new();
    let url = Url::parse(url).map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest("url parse failed".to_string())
    })?;
    let mut req = client.post(url);
    if let Some(headers) = headers {
        for header in headers {
            req = req.header(header.0, header.1.as_str());
        }
    } else {
        req = req.header("Content-Type", "application/json");
    }
    let resp = req.body(payload.clone()).send().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::ThirdRequest(format!("request failed: {:?}", err))
    })?;
    if resp.status().is_success() {
        Ok(resp)
    } else {
        Err(Error::ThirdRequest(format!("request failed ({:?})", resp.status())))
    }
}