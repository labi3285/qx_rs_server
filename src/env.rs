use std::env;
use std::str::FromStr;
use lazy_static::lazy_static;
use dotenv::dotenv;

use crate::err::{Error, Result};
use crate::util::val;

pub static DEFAULT: &'static str = "DEFAULT";

pub fn setup() {
    if cfg!(debug_assertions) {
        let res = dotenv::from_filename(".env.dev").ok();
        match res {
            Some(_) => {
                println!(".env.dev load failed");
            },
            None => {
                println!(".env.dev load failed");
            }
        }
    } else {
        let res = dotenv().ok();
        match res {
            Some(_) => {
                println!(".env load success");
            },
            None => {
                println!(".env load failed");
            }
        }
    }
}


#[allow(unused)]
pub fn str(key: &str) -> Result<String> {
    return _str(key);
}
#[allow(unused)]
pub fn strs(key: &str, sep: &str) -> Result<Vec<String>> {
    let str = _str(key)?;
    let arr: Vec<&str> = str.split(sep).collect();
    return arr.iter().map(|&a| {
        Ok(a.to_string())
    }).collect();
}

#[allow(unused)]
pub fn val<F: FromStr>(key: &str) -> Result<F> {
    let str = _str(key)?;
    return val::str_to_val::<F>(&str);
}


#[allow(unused)]
pub fn vals<F: FromStr>(key: &str, sep: &str) -> Result<Vec<F>> {
    let str = _str(key)?;
    let arr: Vec<&str> = str.split(sep).collect();
    return arr.iter().map(|&a|val::str_to_val::<F>(&(a.to_string()))).collect();
}


fn _str(key: &str) -> Result<String> {
    if !*ENV_EXSIT {
        return Err(Error::Env(".env file not exsit".to_string()));
    }
    let res = env::var(key);
    match res {
        Ok(val) => {
            return Ok(val);
        },
        Err(err) => {
            tracing::error!("{}", err);
            return Err(Error::Env(".env file read failed".to_string()));
        }
    }
}

lazy_static! {
    static ref ENV_EXSIT: bool = {
        let res = dotenv().ok();
        match res {
            Some(_) => {
                return true;
            },
            None => {
                return false;
            }
        }
    };
}


