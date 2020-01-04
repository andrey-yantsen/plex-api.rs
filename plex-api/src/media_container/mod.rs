mod device;
mod settings;

pub use self::device::*;
pub use self::settings::*;
use crate::serde_helpers::{
    bool_from_int, option_bool_from_int, option_comma_separated_to_vec, option_seconds_to_datetime,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainerWrapper {
    #[serde(rename = "MediaContainer")]
    media_container: MediaContainer,
}

impl MediaContainerWrapper {
    pub fn unwrap_media_container(self) -> MediaContainer {
        self.media_container
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    size: Option<u32>,
    total_size: Option<u32>,
    public_address: Option<String>,
    friendly_name: Option<String>,
    identifier: Option<String>,
    machine_identifier: Option<String>,
    #[serde(rename = "Device")]
    devices: Option<Vec<Device>>,
    #[serde(rename = "User")]
    users: Option<Vec<User>>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_camera_upload: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_channel_access: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_media_deletion: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_sharing: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_sync: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    allow_tuners: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    background_processing: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    certificate: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    companion_proxy: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    event_stream: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    hub_search: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    item_clusters: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    media_providers: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    multiuser: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    my_plex: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    my_plex_subscription: Option<bool>,
    country_code: Option<String>,
    #[serde(default, deserialize_with = "option_comma_separated_to_vec")]
    diagnostics: Option<Vec<String>>,
    livetv: Option<u8>,
    my_plex_mapping_state: Option<String>,
    my_plex_signin_state: Option<String>,
    my_plex_username: Option<String>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    owner_features: Option<Vec<String>>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    photo_auto_tag: Option<bool>,
    platform: Option<String>,
    platform_version: Option<String>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    plugin_host: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    request_parameters_in_cookie: Option<bool>,
    read_only_libraries: Option<u16>,
    #[serde(rename = "streamingBrainABRVersion")]
    streaming_brain_abr_version: Option<u8>,
    streaming_brain_version: Option<u8>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    sync: Option<bool>,
    transcoder_active_video_sessions: Option<u8>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    transcoder_audio: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    transcoder_lyrics: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    transcoder_photo: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    transcoder_subtitles: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    transcoder_video: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    updater: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    voice_search: Option<bool>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_bitrates: Option<Vec<u16>>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_qualities: Option<Vec<u8>>,
    #[serde(deserialize_with = "option_comma_separated_to_vec", default)]
    transcoder_video_resolutions: Option<Vec<u16>>,
    #[serde(default, deserialize_with = "option_seconds_to_datetime")]
    updated_at: Option<DateTime<Utc>>,
    version: Option<String>,
    #[serde(rename = "Directory")]
    directories: Option<Vec<Directory>>,
    max_upload_bitrate: Option<u16>,
    max_upload_bitrate_reason: Option<String>,
    max_upload_bitrate_reason_message: Option<String>,
    #[serde(rename = "Setting", default)]
    settings: Option<Vec<Setting>>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    push_notifications: Option<bool>,
}

impl MediaContainer {
    pub fn get_devices(&self) -> Option<Vec<Device>> {
        self.devices.clone()
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        self.users.clone()
    }
}

impl From<MediaContainerWrapper> for MediaContainer {
    fn from(w: MediaContainerWrapper) -> Self {
        w.unwrap_media_container()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Directory {
    count: u16,
    key: String,
    title: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    title: String,
    thumb: String,
    #[serde(deserialize_with = "bool_from_int")]
    protected: bool,
    #[serde(deserialize_with = "bool_from_int")]
    home: bool,
    #[serde(deserialize_with = "bool_from_int")]
    allow_sync: bool,
    #[serde(deserialize_with = "bool_from_int")]
    allow_camera_upload: bool,
    #[serde(deserialize_with = "bool_from_int")]
    allow_channels: bool,
    #[serde(deserialize_with = "bool_from_int")]
    allow_tuners: bool,
    #[serde(deserialize_with = "bool_from_int")]
    allow_subtitle_admin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    restricted: bool,
    filter_all: String,
    filter_movies: String,
    filter_music: String,
    filter_photos: String,
    filter_television: String,
    #[serde(rename = "Server")]
    servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Server {
    id: u32,
    server_id: u32,
    machine_identifier: String,
    name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    last_seen_at: DateTime<Utc>,
    num_libraries: u32,
    #[serde(deserialize_with = "bool_from_int")]
    all_libraries: bool,
    #[serde(deserialize_with = "bool_from_int")]
    owned: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pending: bool,
}
