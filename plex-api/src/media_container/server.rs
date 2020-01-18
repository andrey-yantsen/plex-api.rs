use crate::{Directory, MediaContainer};

use crate::serde_helpers::{
    option_bool_from_int, option_comma_separated_to_vec, option_seconds_to_datetime,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ServerMediaContainer {
    machine_identifier: String,
    #[serde(rename = "Directory")]
    directories: Vec<Directory>,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_camera_upload: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_channel_access: bool,
    #[serde(
        deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything",
        default
    )]
    allow_media_deletion: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_sharing: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_sync: bool,
    #[serde(
        deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything",
        default
    )]
    allow_tuners: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    background_processing: bool,
    #[serde(
        deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything",
        default
    )]
    certificate: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    companion_proxy: bool,
    #[serde(default)]
    country_code: String,
    livetv: u8,
    #[serde(default, deserialize_with = "option_comma_separated_to_vec")]
    diagnostics: Option<Vec<String>>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    media_providers: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    multiuser: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    my_plex: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    my_plex_subscription: Option<bool>,
    my_plex_mapping_state: Option<String>,
    my_plex_signin_state: Option<String>,
    my_plex_username: Option<String>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    owner_features: Option<Vec<String>>,

    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    event_stream: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    hub_search: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    item_clusters: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    photo_auto_tag: bool,

    platform: String,
    platform_version: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    plugin_host: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    request_parameters_in_cookie: bool,
    read_only_libraries: u16,
    #[serde(rename = "streamingBrainABRVersion")]
    streaming_brain_abr_version: u8,
    streaming_brain_version: u8,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    sync: bool,
    transcoder_active_video_sessions: u8,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    transcoder_audio: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    transcoder_lyrics: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    transcoder_photo: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    transcoder_subtitles: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    transcoder_video: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    updater: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    voice_search: bool,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_bitrates: Option<Vec<u16>>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_qualities: Option<Vec<u8>>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_resolutions: Option<Vec<u16>>,
    #[serde(default, deserialize_with = "option_seconds_to_datetime")]
    updated_at: Option<DateTime<Utc>>,
    version: Option<String>,

    max_upload_bitrate: Option<u16>,
    max_upload_bitrate_reason: Option<String>,
    max_upload_bitrate_reason_message: Option<String>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    push_notifications: Option<bool>,

    #[serde(flatten)]
    media_container: MediaContainer,
}

impl ServerMediaContainer {
    pub const fn get_media_container(&self) -> &MediaContainer {
        &self.media_container
    }
}
