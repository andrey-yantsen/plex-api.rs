use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ResourcesMediaContainer {
    #[serde(rename = "Player")]
    pub player: Player,
    #[serde(rename = "@size")]
    pub size: Option<u32>,
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
    #[serde(rename = "@protocolVersion")]
    pub protocol_version: u8,
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
    #[serde(rename = "playqueues")]
    PlayQueues,
    ProviderPlayback,
    Timeline,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(ProtocolCapability);
