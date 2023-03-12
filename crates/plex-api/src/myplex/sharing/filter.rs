use serde::{
    de::{Deserializer, Visitor},
    Deserialize, Serialize,
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    result::Result as StdResult,
};

#[derive(Default, Debug, Clone)]
pub struct SharingFilter {
    pub content_rating: Vec<String>,
    pub exclude_content_rating: Vec<String>,
    pub label: Vec<String>,
    pub exclude_label: Vec<String>,
}

impl<'de> Deserialize<'de> for SharingFilter {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = SharingFilter;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("valid plex filter string")
            }

            fn visit_str<E>(self, value: &str) -> StdResult<SharingFilter, E>
            where
                E: ::serde::de::Error,
            {
                let mut ret = SharingFilter::default();

                if value.is_empty() {
                    return Ok(ret);
                }

                for filter in value.split('|') {
                    let decoded_values =
                        serde_urlencoded::from_str::<Vec<(String, String)>>(filter);
                    if let Ok(decoded_values) = decoded_values {
                        for pairs in decoded_values {
                            match pairs.0.as_str() {
                                "contentRating" => {
                                    ret.content_rating =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "contentRating!" => {
                                    ret.exclude_content_rating =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "label" => {
                                    ret.label = pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "label!" => {
                                    ret.exclude_label =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                _ => {
                                    return Err(::serde::de::Error::invalid_value(
                                        ::serde::de::Unexpected::Str(value),
                                        &self,
                                    ));
                                }
                            }
                        }
                    } else {
                        return Err(::serde::de::Error::invalid_value(
                            ::serde::de::Unexpected::Str(value),
                            &self,
                        ));
                    }
                }

                Ok(ret)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl Serialize for SharingFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for SharingFilter {
    fn to_string(&self) -> String {
        let mut ret: Vec<String> = vec![];

        if !self.content_rating.is_empty() {
            ret.push(format!("contentRating={}", self.content_rating.join("%2C")))
        }

        if !self.exclude_content_rating.is_empty() {
            ret.push(format!(
                "contentRating!={}",
                self.exclude_content_rating.join("%2C")
            ))
        }

        if !self.label.is_empty() {
            ret.push(format!("label={}", self.label.join("%2C")))
        }

        if !self.exclude_label.is_empty() {
            ret.push(format!("label!={}", self.exclude_label.join("%2C")))
        }

        ret.join("|")
    }
}
