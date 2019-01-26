use chrono::{DateTime, Utc};
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use std::fmt::Display;
use std::result::Result;
use std::str::FromStr;

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(u64::from(other)),
            &"zero or one",
        )),
    }
}

pub fn option_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    bool_from_int(deserializer).map(Option::from)
}

pub fn option_seconds_to_datetime<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(Option::from(chrono::serde::ts_seconds::deserialize(d)?))
}

pub fn option_comma_separated_to_vec<'de, D, T>(d: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let v: Vec<T> =
        serde_with::rust::StringWithSeparator::<serde_with::CommaSeparator>::deserialize(d)?;
    Ok(Option::from(v))
}
