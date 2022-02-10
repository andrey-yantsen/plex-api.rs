//! We have a custom deserializer for preferences to avoid nasty untagged
//! enums.
//!
//! Each setting represented in the server's response has a `type` field,
//! which determines what data is stored inside the `value` and `default`
//! fields. I know two ways how we parse it using only derives:
//!
//! 1. Put both fields inside a tagged enum. This leads to complications
//!    when we expose the API for changing the value.
//! 2. Put each field inside an untagged enum and just ignore the `type`.
//!    This could lead to unexpected errors for the API users.
//!
//! If you know how to simplify all this — I'd be happy to hear from you :)

use super::Setting;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::str::FromStr;
use thiserror::Error;

impl<'de> Deserialize<'de> for Setting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Id,
            Label,
            Summary,
            Hidden,
            Advanced,
            Group,
            Value,
            Default,
            r#Type,
            EnumValues,
        }

        struct SettingVisitor;
        impl<'de> Visitor<'de> for SettingVisitor {
            type Value = Setting;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Setting")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut label = None;
                let mut summary = None;
                let mut hidden = None;
                let mut advanced = None;
                let mut group = None;
                let mut value = None;
                let mut default = None;
                let mut r#type = None;
                let mut enum_values: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Label => {
                            if label.is_some() {
                                return Err(de::Error::duplicate_field("label"));
                            }
                            label = Some(map.next_value()?);
                        }
                        Field::Summary => {
                            if summary.is_some() {
                                return Err(de::Error::duplicate_field("summary"));
                            }
                            summary = Some(map.next_value()?);
                        }
                        Field::Hidden => {
                            if hidden.is_some() {
                                return Err(de::Error::duplicate_field("hidden"));
                            }
                            hidden = Some(map.next_value()?);
                        }
                        Field::Advanced => {
                            if advanced.is_some() {
                                return Err(de::Error::duplicate_field("advanced"));
                            }
                            advanced = Some(map.next_value()?);
                        }
                        Field::Group => {
                            if group.is_some() {
                                return Err(de::Error::duplicate_field("group"));
                            }
                            group = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Default => {
                            if default.is_some() {
                                return Err(de::Error::duplicate_field("default"));
                            }
                            default = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value()?);
                        }
                        Field::EnumValues => {
                            if enum_values.is_some() {
                                return Err(de::Error::duplicate_field("enumValues"));
                            }
                            enum_values = Some(map.next_value()?);
                        }
                    }
                }

                let id: String = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let label: String = label.ok_or_else(|| de::Error::missing_field("label"))?;
                let summary: String = summary.ok_or_else(|| de::Error::missing_field("summary"))?;
                let hidden: bool = hidden.ok_or_else(|| de::Error::missing_field("hidden"))?;
                let advanced: bool =
                    advanced.ok_or_else(|| de::Error::missing_field("advanced"))?;
                let group: String = group.ok_or_else(|| de::Error::missing_field("group"))?;
                let r#type: String = r#type.ok_or_else(|| de::Error::missing_field("type"))?;
                let value: serde_json::Value =
                    value.ok_or_else(|| de::Error::missing_field("value"))?;
                let default: serde_json::Value =
                    default.ok_or_else(|| de::Error::missing_field("default"))?;

                let mut suggested_values: Option<Vec<super::SettingEnumValue>> = None;

                if let Some(serde_json::Value::String(enum_values)) = enum_values {
                    let suggested_values_tmp: Result<Vec<_>, _> = enum_values
                        .split('|')
                        .map(super::SettingEnumValue::from_str)
                        .collect();

                    if let Ok(ok) = suggested_values_tmp {
                        suggested_values = Some(ok);
                    } else {
                        return Err(de::Error::custom("invalid value inside enumValues"));
                    }
                }

                match r#type.as_str() {
                    "bool" => Ok(Setting {
                        id,
                        label,
                        summary,
                        hidden,
                        advanced,
                        group,
                        value: super::Value::Bool(
                            value
                                .as_bool()
                                .ok_or_else(|| de::Error::custom("bool expected"))?,
                        ),
                        default: super::Value::Bool(
                            default
                                .as_bool()
                                .ok_or_else(|| de::Error::custom("bool expected"))?,
                        ),
                        suggested_values,
                    }),
                    "int" => Ok(Setting {
                        id,
                        label,
                        summary,
                        hidden,
                        advanced,
                        group,
                        value: super::Value::Int(
                            value
                                .as_i64()
                                .ok_or_else(|| de::Error::custom("int expected"))?,
                        ),
                        default: super::Value::Int(
                            default
                                .as_i64()
                                .ok_or_else(|| de::Error::custom("int expected"))?,
                        ),
                        suggested_values,
                    }),
                    "text" => Ok(Setting {
                        id,
                        label,
                        summary,
                        hidden,
                        advanced,
                        group,
                        value: super::Value::Text(
                            value
                                .as_str()
                                .ok_or_else(|| de::Error::custom("string expected"))?
                                .to_owned(),
                        ),
                        default: super::Value::Text(
                            default
                                .as_str()
                                .ok_or_else(|| de::Error::custom("string expected"))?
                                .to_owned(),
                        ),
                        suggested_values,
                    }),
                    "double" => Ok(Setting {
                        id,
                        label,
                        summary,
                        hidden,
                        advanced,
                        group,
                        value: super::Value::Double(
                            value
                                .as_f64()
                                .ok_or_else(|| de::Error::custom("double expected"))?,
                        ),
                        default: super::Value::Double(
                            default
                                .as_f64()
                                .ok_or_else(|| de::Error::custom("double expected"))?
                                .to_owned(),
                        ),
                        suggested_values,
                    }),
                    _ => Err(de::Error::unknown_variant(
                        &r#type,
                        &["bool", "int", "text", "double"],
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &[
            "id",
            "label",
            "summary",
            "hidden",
            "advanced",
            "group",
            "value",
            "default",
            "type",
            "enumValues",
        ];
        deserializer.deserialize_struct("Setting", FIELDS, SettingVisitor)
    }
}

#[derive(Error, Debug)]
pub enum SettingError {
    #[error("Provided `enumValues` string incorrectly formatted")]
    IncorrectEnumValuesFormat,
}

impl std::str::FromStr for super::SettingEnumValue {
    type Err = SettingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<_> = s.split(':').collect();

        match value.len() {
            1 => Ok(super::SettingEnumValue {
                value: String::from(s),
                hint: String::from(s),
            }),
            2 => Ok(super::SettingEnumValue {
                value: String::from(value[0]),
                hint: String::from(value[1]),
            }),
            _ => Err(SettingError::IncorrectEnumValuesFormat),
        }
    }
}
