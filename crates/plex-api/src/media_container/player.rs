use super::MediaContainer;
use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ResourcesMediaContainer {
    #[serde(rename = "Player", default)]
    pub players: Vec<Player>,
    #[serde(flatten)]
    pub media_container: MediaContainer,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Player {
    #[serde(rename = "@machineIdentifier")]
    pub machine_identifier: String,
    #[serde(rename = "@product")]
    pub product: String,
    #[serde(rename = "@protocol")]
    pub protocol: String,
    #[serde(rename = "@deviceClass")]
    pub device_class: DeviceClass,
    #[serde(rename = "@platform")]
    pub platform: String,
    #[serde(rename = "@platformVersion")]
    pub platform_version: String,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, ProtocolCapability>")]
    #[serde(rename = "@protocolCapabilities")]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ProtocolCapability {
    Mirror,
    Playback,
    Playqueues,
    ProviderPlayback,
    Timeline,
}

derive_fromstr_from_deserialize!(ProtocolCapability);
