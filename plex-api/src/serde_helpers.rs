use std::fmt::Display;
use std::result::Result;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::fmt;
use std::iter::FromIterator;

pub(crate) fn option_bool_from_anything<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
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

pub(crate) fn duration_from_seconds<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct DurationVisitor;

    impl<'de> Visitor<'de> for DurationVisitor {
        type Value = chrono::Duration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a JSON number")
        }

        #[inline]
        fn visit_u64<E>(self, value: u64) -> Result<chrono::Duration, E> {
            Ok(chrono::Duration::milliseconds(value as i64))
        }
    }

    deserializer.deserialize_u64(DurationVisitor)
}

pub(crate) fn option_duration_from_seconds<'de, D>(
    d: D,
) -> Result<Option<chrono::Duration>, D::Error>
where
    D: de::Deserializer<'de>,
{
    duration_from_seconds(d).map(Option::from)
}

pub(crate) fn date_from_iso<'de, D>(d: D) -> Result<chrono::Date<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let date = String::deserialize(d)?;
    let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d");
    if let Ok(date) = date {
        Ok(chrono::Date::from_utc(date, Utc))
    } else {
        Err(serde::de::Error::custom("failed to convert date"))
    }
}

pub(crate) fn option_date_from_iso<'de, D>(d: D) -> Result<Option<chrono::Date<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    date_from_iso(d).map(Option::from)
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
