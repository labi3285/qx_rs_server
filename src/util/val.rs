

use std::any::type_name;
use std::str::FromStr;

use crate::err::{Error, Result};

#[allow(unused)]
pub fn str_to_val<F: FromStr>(v: &String) -> Result<F> {
    let mut v = &v[..];
    if type_name::<F>() == type_name::<bool>() {
        if v == "1" || v == "true" || v == "T" || v == "TRUE" || v == "yes" || v == "YES" {
            v = "true";
        } else if v == "0" || v == "false" || v == "F" || v == "FALSE" || v == "NO" || v == "NO" {
            v = "false";
        } else {
            let err = format!("parsing bool failed:{:?}", v);
            tracing::error!("{}", err);
            return Err(Error::Parsing(err));
        }
    }
    let res = v.parse::<F>();
    match res {
        Ok(val) => {
            return Ok(val);
        },
        Err(_) => {
            let err = format!("parsing bool failed:{:?}", v);
            tracing::error!("{}", err);
            return Err(Error::Parsing(err));
        }
    }
}

