use crate::serde_helpers::option_pipe_separated_to_vec;
use crate::MediaContainer;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SettingsMediaContainer {
    #[serde(rename = "Setting")]
    settings: Vec<Setting>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

impl SettingsMediaContainer {
    pub fn get_media_container(self) -> MediaContainer {
        self.media_container
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
pub enum Payload {
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
        // TODO: use f64, current problem that `TranscoderH264MinimumCRF` stored as string in JSON
        default: String,
        value: String,
    },
}

pub enum SettingValue {
    Bool(bool),
    Int(i32),
    Text(String),
    Double(f64),
}
