pub(crate) mod devices;
pub(crate) mod player;
pub(crate) mod server;
pub(crate) mod users;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    pub size: Option<u32>,
    pub total_size: Option<u32>,
    pub public_address: Option<String>,
    pub friendly_name: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainerWrapper<T> {
    #[serde(rename = "MediaContainer")]
    pub media_container: T,
}
