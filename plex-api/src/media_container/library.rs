use crate::MediaContainer;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct LibraryMediaContainer {
    allow_sync: bool,
    art: Option<String>,
    content: Option<String>,
    identifier: String,
    media_tag_prefix: String,
    media_tag_version: u64,
    title1: Option<String>,
    title2: Option<String>,
    #[serde(rename = "Directory")]
    pub(crate) directory: Option<Vec<DirectoryMediaContainer>>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct LibraryMediaContainerOuter {
    #[serde(rename = "MediaContainer")]
    media_container: LibraryMediaContainer,
}

impl From<LibraryMediaContainerOuter> for LibraryMediaContainer {
    fn from(mc: LibraryMediaContainerOuter) -> Self {
        mc.media_container
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
enum DirectoryType {
    Movie,
    Show,
    #[serde(rename = "artist")]
    Music,
    Photo,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct DirectoryLocation {
    id: u32,
    path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DirectoryMediaContainer {
    pub(crate) key: String,
    pub(crate) title: String,
    art: String,
    allow_sync: bool,
    composite: String,
    filters: bool,
    refreshing: bool,
    thumb: String,
    #[serde(rename = "type")]
    directory_type: String,
    agent: String,
    scanner: String,
    language: String,
    uuid: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    scanned_at: DateTime<Utc>,
    content: bool,
    directory: bool,
    content_changed_at: u64,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    hidden: bool,
    #[serde(default)]
    #[serde(rename = "Location")]
    location: Vec<DirectoryLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_auto_photo_tags: Option<bool>,
}
