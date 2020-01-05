use crate::MediaContainer;

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
        #[serde(rename = "enumValues")]
        enum_values: Option<String>, // TODO: decode values to hash/vec
    },
    Text {
        default: String,
        value: String,
        #[serde(rename = "enumValues")]
        enum_values: Option<String>, // TODO: decode values to hash/vec
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
