use std::fmt::Display;
use std::result::Result;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::de::{self, Deserializer};
use std::iter::FromIterator;

pub(crate) fn option_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    serde_aux::prelude::deserialize_bool_from_anything(deserializer).map(Option::from)
}

pub(crate) fn option_int_from_string<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    serde_aux::prelude::deserialize_number_from_string::<T, D>(deserializer).map(Option::from)
}

pub(crate) fn option_seconds_to_datetime<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    chrono::serde::ts_seconds::deserialize(d).map(Option::from)
}

pub(crate) fn option_comma_separated_to_vec<'de, D, T, V>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<V>,
    V: FromStr,
    V::Err: Display,
{
    serde_with::rust::StringWithSeparator::<serde_with::CommaSeparator>::deserialize::<D, T, V>(d)
        .map(Option::from)
}

pub(crate) fn option_pipe_separated_to_vec<'de, D, T, V>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<V>,
    V: FromStr,
    V::Err: Display,
{
    serde_with::rust::StringWithSeparator::<PipeSeparator>::deserialize::<D, T, V>(d)
        .map(Option::from)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PipeSeparator;

impl serde_with::Separator for PipeSeparator {
    #[inline]
    fn separator() -> &'static str {
        "|"
    }
}
