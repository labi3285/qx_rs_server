#[allow(unused)]

use crate::env;
use crate::err::{Error, Result};

use axum::Router;


pub async fn serve(router: Router) -> Result<()> {
    let port = env::str("APP.PORT")?;
    let addr = format!("0.0.0.0:{}", port);
    let res = tokio::net::TcpListener::bind(&addr).await;
    match res {
        Ok(listener) => {
            tracing::info!("server started at: http://{}", addr);
            let res = axum::serve(listener, router).await;
            match res {
                Ok(_) => {
                    return Ok(())
                },
                Err(err) => {
                    tracing::error!("{}", err);
                    return Err(Error::ServerStart(format!("server serve failed:{:?}", err)));
                }
            }
        },
        Err(err) => {
            tracing::error!("{}", err);
            return Err(Error::ServerStart(format!("server bind addr failed:{:?}", err)));
        },
    }
}

