mod device;

pub use self::device::*;
use crate::serde_helpers::{
    bool_from_int, option_bool_from_int, option_comma_separated_to_vec, option_seconds_to_datetime,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MediaContainer {
    size: Option<u32>,
    #[serde(rename = "totalSize")]
    total_size: Option<u32>,
    #[serde(rename = "publicAddress")]
    public_address: Option<String>,
    #[serde(rename = "friendlyName")]
    friendly_name: Option<String>,
    identifier: Option<String>,
    #[serde(rename = "machineIdentifier")]
    machine_identifier: Option<String>,
    #[serde(rename = "Device")]
    devices: Option<Vec<Device>>,
    #[serde(rename = "User")]
    users: Option<Vec<User>>,
    #[serde(
        rename = "allowCameraUpload",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_camera_upload: Option<bool>,
    #[serde(
        rename = "allowChannelAccess",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_channel_access: Option<bool>,
    #[serde(
        rename = "allowMediaDeletion",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_media_deletion: Option<bool>,
    #[serde(
        rename = "allowSharing",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_sharing: Option<bool>,
    #[serde(
        rename = "allowSync",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_sync: Option<bool>,
    #[serde(
        rename = "allowTuners",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    allow_tuners: Option<bool>,
    #[serde(
        rename = "backgroundProcessing",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    background_processing: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    certificate: Option<bool>,
    #[serde(
        rename = "companionProxy",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    companion_proxy: Option<bool>,
    #[serde(
        rename = "eventStream",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    event_stream: Option<bool>,
    #[serde(
        rename = "hubSearch",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    hub_search: Option<bool>,
    #[serde(
        rename = "itemClusters",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    item_clusters: Option<bool>,
    #[serde(
        rename = "mediaProviders",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    media_providers: Option<bool>,
    #[serde(
        rename = "multiuser",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    multiuser: Option<bool>,
    #[serde(rename = "myPlex", default, deserialize_with = "option_bool_from_int")]
    my_plex: Option<bool>,
    #[serde(
        rename = "myPlexSubscription",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    my_plex_subscription: Option<bool>,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    #[serde(default, deserialize_with = "option_comma_separated_to_vec")]
    diagnostics: Option<Vec<String>>,
    livetv: Option<u8>,
    #[serde(rename = "myPlexMappingState")]
    my_plex_mapping_state: Option<String>,
    #[serde(rename = "myPlexSigninState")]
    my_plex_signin_state: Option<String>,
    #[serde(rename = "myPlexUsername")]
    my_plex_username: Option<String>,
    #[serde(
        rename = "ownerFeatures",
        deserialize_with = "option_comma_separated_to_vec",
        default
    )]
    owner_features: Option<Vec<String>>,
    #[serde(
        rename = "photoAutoTag",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    photo_auto_tag: Option<bool>,
    platform: Option<String>,
    #[serde(rename = "platformVersion")]
    platform_version: Option<String>,
    #[serde(
        rename = "pluginHost",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    plugin_host: Option<bool>,
    #[serde(
        rename = "requestParametersInCookie",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    request_parameters_in_cookie: Option<bool>,
    #[serde(rename = "readOnlyLibraries")]
    read_only_libraries: Option<u16>,
    #[serde(rename = "streamingBrainABRVersion")]
    streaming_brain_abr_version: Option<u8>,
    #[serde(rename = "streamingBrainVersion")]
    streaming_brain_version: Option<u8>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    sync: Option<bool>,
    #[serde(rename = "transcoderActiveVideoSessions")]
    transcoder_active_video_sessions: Option<u8>,
    #[serde(
        rename = "transcoderAudio",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    transcoder_audio: Option<bool>,
    #[serde(
        rename = "transcoderLyrics",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    transcoder_lyrics: Option<bool>,
    #[serde(
        rename = "transcoderPhoto",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    transcoder_photo: Option<bool>,
    #[serde(
        rename = "transcoderSubtitles",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    transcoder_subtitles: Option<bool>,
    #[serde(
        rename = "transcoderVideo",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    transcoder_video: Option<bool>,
    #[serde(default, deserialize_with = "option_bool_from_int")]
    updater: Option<bool>,
    #[serde(
        rename = "voiceSearch",
        default,
        deserialize_with = "option_bool_from_int"
    )]
    voice_search: Option<bool>,
    #[serde(
        rename = "transcoderVideoBitrates",
        deserialize_with = "option_comma_separated_to_vec",
        default
    )]
    transcoder_video_bitrates: Option<Vec<u16>>,
    #[serde(
        rename = "transcoderVideoQualities",
        deserialize_with = "option_comma_separated_to_vec",
        default
    )]
    transcoder_video_qualities: Option<Vec<u8>>,
    #[serde(
        rename = "transcoderVideoResolutions",
        deserialize_with = "option_comma_separated_to_vec",
        default
    )]
    transcoder_video_resolutions: Option<Vec<u16>>,
    #[serde(
        default,
        rename = "updatedAt",
        deserialize_with = "option_seconds_to_datetime"
    )]
    updated_at: Option<DateTime<Utc>>,
    version: Option<String>,
    #[serde(rename = "Directory")]
    directories: Option<Vec<Directory>>,
}

impl MediaContainer {
    pub fn get_devices(&self) -> Option<Vec<Device>> {
        self.devices.clone()
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        self.users.clone()
    }

    pub fn get_directories(&self) -> Option<Vec<Directory>> {
        self.directories.clone()
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
pub struct User {
    id: u32,
    title: String,
    thumb: String,
    #[serde(deserialize_with = "bool_from_int")]
    protected: bool,
    #[serde(deserialize_with = "bool_from_int")]
    home: bool,
    #[serde(rename = "allowSync", deserialize_with = "bool_from_int")]
    allow_sync: bool,
    #[serde(rename = "allowCameraUpload", deserialize_with = "bool_from_int")]
    allow_camera_upload: bool,
    #[serde(rename = "allowChannels", deserialize_with = "bool_from_int")]
    allow_channels: bool,
    #[serde(rename = "allowTuners", deserialize_with = "bool_from_int")]
    allow_tuners: bool,
    #[serde(rename = "allowSubtitleAdmin", deserialize_with = "bool_from_int")]
    allow_subtitle_admin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    restricted: bool,
    #[serde(rename = "filterAll")]
    filter_all: String,
    #[serde(rename = "filterMovies")]
    filter_movies: String,
    #[serde(rename = "filterMusic")]
    filter_music: String,
    #[serde(rename = "filterPhotos")]
    filter_photos: String,
    #[serde(rename = "filterTelevision")]
    filter_television: String,
    #[serde(rename = "Server")]
    servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Server {
    id: u32,
    #[serde(rename = "serverId")]
    server_id: u32,
    #[serde(rename = "machineIdentifier")]
    machine_identifier: String,
    name: String,
    #[serde(rename = "lastSeenAt", with = "chrono::serde::ts_seconds")]
    last_seen_at: DateTime<Utc>,
    #[serde(rename = "numLibraries")]
    num_libraries: u32,
    #[serde(rename = "allLibraries", deserialize_with = "bool_from_int")]
    all_libraries: bool,
    #[serde(deserialize_with = "bool_from_int")]
    owned: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pending: bool,
}
