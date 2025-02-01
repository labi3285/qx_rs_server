#[allow(unused)]


use serde::{self, Serialize};

use crate::util::arr;

pub fn codes_formatted_serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let val = arr::parse_codes(value);
    val.serialize(serializer)
}

pub fn ids_formatted_serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let val = arr::parse_ids(value);
    val.serialize(serializer)
}

pub fn json_strs_formatted_serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(v) = value {
        let arr = serde_json::from_str::<Vec<String>>(v).unwrap();
        arr.serialize(serializer)
    } else {
        Option::<Vec<String>>::None.serialize(serializer)
    }
}

