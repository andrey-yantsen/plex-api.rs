use serde::Deserialize;
use serde_with::{rust::StringWithSeparator, CommaSeparator};
use strum::EnumString;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DevicesMediaContainer {
    #[serde(rename = "Device", default)]
    pub devices: Vec<Device>,

    // TODO: The rest of the fields should belong to MediaContainer struct, but
    // quick-qml (and serde-xml-rs) has some issues with `flatten`.
    // https://github.com/tafia/quick-xml/issues/226
    #[serde(default)]
    pub size: Option<i32>,
    pub public_address: Option<String>,
    // #[serde(flatten)]
    // pub media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub product: String,
    pub public_address: String,
    pub product_version: String,
    pub platform: String,
    pub platform_version: String,
    pub device: String,
    pub model: Option<String>,
    pub vendor: Option<String>,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub provides: Vec<Feature>,
    pub client_identifier: String,
    pub version: Option<String>,
    pub id: Option<u32>,
    pub token: Option<String>,
    pub access_token: Option<String>,
    #[serde(with = "time::serde::timestamp")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub last_seen_at: OffsetDateTime,
    #[serde(
        deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize",
        default
    )]
    pub screen_resolution: Vec<String>,
    #[serde(
        deserialize_with = "serde_with::rust::string_empty_as_none::deserialize",
        default
    )]
    pub screen_density: Option<u16>,
    #[serde(rename = "Connection")]
    pub connections: Option<Vec<Connection>>,
    pub https_required: Option<bool>,
    pub synced: Option<bool>,
    pub relay: Option<bool>,
    pub public_address_matches: Option<bool>,
    pub presence: Option<bool>,
    pub owned: Option<bool>,
    #[serde(rename = "SyncList")]
    pub sync_list: Option<SyncList>,
    #[serde(default)]
    pub auth_token: String,
    pub dns_rebinding_protection: Option<bool>,
    pub nat_loopback_supported: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SyncList {
    pub items_complete_count: u32,
    pub total_size: u64,
    pub version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Connection {
    #[serde(with = "http_serde::uri")]
    pub uri: http::Uri,
    pub protocol: Option<String>,
    pub address: Option<String>,
    pub port: Option<u32>,
    pub local: Option<bool>,
    pub relay: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, EnumString)]
#[serde(rename_all = "kebab-case")]
pub enum Feature {
    Server,
    Client,
    Controller,
    Player,
    PubsubPlayer,
    ProviderPlayback,
    SyncTarget,
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}
