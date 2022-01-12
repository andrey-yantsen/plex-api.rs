use serde::Deserialize;

use super::MediaContainer;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(flatten)]
    pub media_container: MediaContainer,
}
