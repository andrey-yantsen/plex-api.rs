use serde::{
    de::{Error as DeError, SeqAccess, Visitor},
    Deserializer,
};
use serde_aux::prelude::{
    deserialize_bool_from_anything, deserialize_option_number_from_string,
    deserialize_string_from_number,
};
use serde_with::{formats::Separator, DeserializeAs};
use std::{
    fmt::{self, Display},
    marker::PhantomData,
    str::FromStr,
};
use time::OffsetDateTime;

pub(crate) fn optional_boolish<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_bool_from_anything(deserializer)?))
}

pub(crate) fn deserialize_option_string_from_number<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_string_from_number(deserializer)?))
}

pub(crate) fn deserialize_option_datetime_from_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_option_number_from_string(deserializer)?
        .map(OffsetDateTime::from_unix_timestamp)
        .transpose()
        .map_err(DeError::custom)
}

pub(crate) struct StringWithSeparatorOrList<Sep, T>(PhantomData<(Sep, T)>);

impl<'de, SEPARATOR, I, T> DeserializeAs<'de, I> for StringWithSeparatorOrList<SEPARATOR, T>
where
    SEPARATOR: Separator,
    I: FromIterator<T>,
    T: FromStr,
    T::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<I, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper<SEPARATOR, I, T>(PhantomData<(SEPARATOR, I, T)>);

        impl<'de, SEPARATOR, I, T> Visitor<'de> for Helper<SEPARATOR, I, T>
        where
            SEPARATOR: Separator,
            I: FromIterator<T>,
            T: FromStr,
            T::Err: Display,
        {
            type Value = I;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value.is_empty() {
                    Ok(None.into_iter().collect())
                } else {
                    value
                        .split(SEPARATOR::separator())
                        .map(FromStr::from_str)
                        .collect::<Result<_, _>>()
                        .map_err(DeError::custom)
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut ret: Vec<T> = Vec::with_capacity(seq.size_hint().unwrap_or_default());

                while let Some(item) = seq.next_element::<&str>()? {
                    ret.push(FromStr::from_str(item).map_err(DeError::custom)?)
                }

                Ok(ret.into_iter().collect())
            }
        }

        deserializer.deserialize_any(Helper::<SEPARATOR, I, T>(PhantomData))
    }
}
