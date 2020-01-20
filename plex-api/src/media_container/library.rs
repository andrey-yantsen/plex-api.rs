use crate::MediaContainer;

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
    directory: Option<Vec<DirectoryMediaContainer>>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub(crate) struct LibraryMediaContainerOuter {
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
#[serde(rename_all = "camelCase")]
pub struct DirectoryMediaContainer {
    key: String,
    title: String,
}
