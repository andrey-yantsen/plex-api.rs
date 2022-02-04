use super::MediaContainer;
use serde::Deserialize;
use serde_with::{rust::StringWithSeparator, CommaSeparator};
use strum::EnumString;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ResourcesMediaContainer {
    #[serde(rename = "Player", default)]
    pub players: Vec<Player>,
    #[serde(flatten)]
    pub media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub machine_identifier: String,
    pub product: String,
    pub protocol: String,
    pub device_class: DeviceClass,
    pub platform: String,
    pub platform_version: String,
    pub title: String,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub protocol_capabilities: Vec<ProtocolCapability>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceClass {
    Stb,
    Phone,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum ProtocolCapability {
    Mirror,
    Playback,
    Playqueues,
    ProviderPlayback,
    Timeline,
}
