use crate::serde_helpers::option_pipe_separated_to_vec;
use crate::{MediaContainer, PlexApiError};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SettingsMediaContainer {
    #[serde(
        rename = "Setting",
        deserialize_with = "deserialize_settings_as_hashmap"
    )]
    settings: HashMap<String, Setting>,
    #[serde(flatten)]
    media_container: MediaContainer,
    #[serde(skip)]
    updated: HashMap<String, Setting>,
}

impl SettingsMediaContainer {
    pub fn get(&self, name: &str) -> crate::Result<&Setting> {
        if self.updated.contains_key(name) {
            Ok(&self.updated[name])
        } else if self.settings.contains_key(name) {
            Ok(&self.settings[name])
        } else {
            Err(PlexApiError::UnknownSettingRequested {
                key: String::from(name),
                known: self
                    .settings
                    .keys()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join(", "),
            })
        }
    }

    pub fn set(&mut self, name: &str, value: SettingValue) -> crate::Result<()> {
        match self.settings.get(name) {
            Some(current_value) => {
                let mut new_value = current_value.clone();
                if let Err(e) = new_value.set(value) {
                    Err(e)
                } else {
                    self.updated.insert(String::from(name), new_value);
                    Ok(())
                }
            }
            None => Err(PlexApiError::UnknownSettingRequested {
                key: String::from(name),
                known: self
                    .settings
                    .keys()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join(", "),
            }),
        }
    }

    pub const fn get_changed(&self) -> &HashMap<String, Setting> {
        &self.updated
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub(crate) struct SettingsMediaContainerOuter {
    #[serde(rename = "MediaContainer")]
    media_container: SettingsMediaContainer,
}

impl From<SettingsMediaContainerOuter> for SettingsMediaContainer {
    fn from(mc: SettingsMediaContainerOuter) -> Self {
        mc.media_container
    }
}

// TODO: enable `serde(deny_unknown_fields)`
#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
    id: String,
    label: String,
    summary: String,
    hidden: bool,
    advanced: bool,
    group: String,
    #[serde(flatten)]
    payload: Payload,
}

impl Setting {
    fn set(&mut self, new_value: SettingValue) -> crate::Result<()> {
        self.payload.set(new_value)
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_value(&self) -> SettingValue {
        SettingValue::from(&self.payload)
    }
}

#[derive(Error, Debug)]
pub enum SettingParsingError {
    #[error("Provided string incorrectly formatted")]
    IncorrectFormat,
    #[error("Unable to convert provided key to integer: {source}")]
    UnableToParseInt {
        #[from]
        source: std::num::ParseIntError,
    },
}

#[derive(Debug, Clone)]
pub struct SettingEnumValueString(String, String);

impl std::str::FromStr for SettingEnumValueString {
    type Err = SettingParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<_> = s.split(':').collect();

        match value.len() {
            1 => Ok(SettingEnumValueString(String::from(s), String::from(s))),
            2 => Ok(SettingEnumValueString(
                String::from(value[0]),
                String::from(value[1]),
            )),
            _ => Err(SettingParsingError::IncorrectFormat),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingEnumValueInt(i32, String);

impl std::str::FromStr for SettingEnumValueInt {
    type Err = SettingParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<_> = s.split(':').collect();

        match value.len() {
            1 => Ok(SettingEnumValueInt(
                String::from(s).parse()?,
                String::from(s),
            )),
            2 => Ok(SettingEnumValueInt(
                value[0].parse()?,
                String::from(value[1]),
            )),
            _ => Err(SettingParsingError::IncorrectFormat),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum Payload {
    Bool {
        default: bool,
        value: bool,
    },
    Int {
        default: i32,
        value: i32,
        #[serde(
            rename = "enumValues",
            deserialize_with = "option_pipe_separated_to_vec",
            default
        )]
        enum_values: Option<Vec<SettingEnumValueInt>>,
    },
    Text {
        default: String,
        value: String,
        #[serde(
            rename = "enumValues",
            deserialize_with = "option_pipe_separated_to_vec",
            default
        )]
        enum_values: Option<Vec<SettingEnumValueString>>,
    },
    Double {
        #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
        default: f64,
        #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
        value: f64,
    },
}

impl Payload {
    fn set(&mut self, new_value: SettingValue) -> crate::Result<()> {
        match self {
            Payload::Bool {
                value: ref mut current_value,
                ..
            } => match new_value {
                SettingValue::Bool(value) => {
                    *current_value = value;
                    Ok(())
                }
                _ => Err(PlexApiError::ExpectedSettingValueBool {
                    provided: new_value,
                }),
            },
            Payload::Int {
                value: ref mut current_value,
                ..
            } => match new_value {
                SettingValue::Int(value) => {
                    *current_value = value;
                    Ok(())
                }
                _ => Err(PlexApiError::ExpectedSettingValueInt {
                    provided: new_value,
                }),
            },
            Payload::Text {
                value: ref mut current_value,
                ..
            } => match new_value {
                SettingValue::Text(value) => {
                    *current_value = value;
                    Ok(())
                }
                _ => Err(PlexApiError::ExpectedSettingValueText {
                    provided: new_value,
                }),
            },
            Payload::Double {
                value: ref mut current_value,
                ..
            } => match new_value {
                SettingValue::Double(value) => {
                    *current_value = value;
                    Ok(())
                }
                _ => Err(PlexApiError::ExpectedSettingValueDouble {
                    provided: new_value,
                }),
            },
        }
    }
}

#[derive(Debug)]
pub enum SettingValue {
    Bool(bool),
    Int(i32),
    Text(String),
    Double(f64),
}

impl From<bool> for SettingValue {
    fn from(v: bool) -> Self {
        SettingValue::Bool(v)
    }
}

impl From<i32> for SettingValue {
    fn from(v: i32) -> Self {
        SettingValue::Int(v)
    }
}

impl From<String> for SettingValue {
    fn from(v: String) -> Self {
        SettingValue::Text(v)
    }
}

impl From<&str> for SettingValue {
    fn from(v: &str) -> Self {
        SettingValue::Text(String::from(v))
    }
}

impl From<f64> for SettingValue {
    fn from(v: f64) -> Self {
        SettingValue::Double(v)
    }
}

impl From<&Payload> for SettingValue {
    fn from(v: &Payload) -> Self {
        match v {
            Payload::Bool { value, .. } => SettingValue::Bool(*value),
            Payload::Int { value, .. } => SettingValue::Int(*value),
            Payload::Text { value, .. } => SettingValue::Text(value.to_string()),
            Payload::Double { value, .. } => SettingValue::Double(*value),
        }
    }
}

impl ToString for SettingValue {
    fn to_string(&self) -> String {
        match self {
            SettingValue::Bool(v) => v.to_string(),
            SettingValue::Int(v) => v.to_string(),
            SettingValue::Text(v) => v.to_string(),
            SettingValue::Double(v) => v.to_string(),
        }
    }
}

pub fn deserialize_settings_as_hashmap<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Setting>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;

    let mut map = HashMap::new();
    for item in Vec::<Setting>::deserialize(deserializer)? {
        map.insert((&item.id).to_string(), item);
    }
    Ok(map)
}
