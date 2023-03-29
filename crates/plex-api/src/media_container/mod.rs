pub mod devices;
pub mod home;
pub mod player;
pub mod preferences;
pub mod server;
pub mod users;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    pub size: Option<u32>,
    pub total_size: Option<u32>,
    pub public_address: Option<String>,
    pub friendly_name: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaContainerWrapper<T> {
    #[serde(rename = "MediaContainer")]
    pub(crate) media_container: T,
}
