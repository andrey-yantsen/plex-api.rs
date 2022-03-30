mod feature;

pub use self::feature::Feature;
use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{rust::StringWithSeparator, CommaSeparator};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub size: u32,
    pub allow_camera_upload: bool,
    pub allow_channel_access: bool,
    pub allow_media_deletion: bool,
    pub allow_sharing: bool,
    pub allow_sync: bool,
    pub allow_tuners: bool,
    pub background_processing: bool,
    pub certificate: Option<bool>,
    pub companion_proxy: bool,
    pub country_code: Option<String>,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub diagnostics: Vec<Diagnostics>,
    pub event_stream: bool,
    pub friendly_name: String,
    pub livetv: u8,
    pub machine_identifier: String,
    pub music_analysis: Option<u8>,
    pub my_plex: bool,
    pub my_plex_mapping_state: MappingState,
    pub my_plex_mapping_error: Option<MappingError>,
    pub my_plex_signin_state: MyPlexSignInState,
    pub my_plex_subscription: bool,
    pub my_plex_username: Option<String>,
    pub offline_transcode: Option<u8>,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub owner_features: Vec<Feature>,
    pub photo_auto_tag: bool,
    pub platform: String,
    pub platform_version: String,
    pub plugin_host: bool,
    pub push_notifications: bool,
    pub read_only_libraries: bool,
    pub start_state: Option<String>,
    #[serde(rename = "streamingBrainABRVersion")]
    pub streaming_brain_abr_version: u8,
    pub streaming_brain_version: u8,
    pub sync: bool,
    pub transcoder_active_video_sessions: u8,
    pub transcoder_audio: bool,
    pub transcoder_lyrics: bool,
    pub transcoder_subtitles: bool,
    pub transcoder_video: bool,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub transcoder_video_bitrates: Vec<u32>,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub transcoder_video_qualities: Vec<u8>,
    #[serde(deserialize_with = "StringWithSeparator::<CommaSeparator>::deserialize")]
    pub transcoder_video_resolutions: Vec<u16>,
    pub updated_at: i64,
    pub updater: bool,
    pub version: String,
    pub voice_search: bool,
    #[serde(rename = "MediaProvider")]
    pub media_provider: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Diagnostics {
    Logs,
    Databases,
    StreamingLogs,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

derive_fromstr_from_deserialize!(Diagnostics);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MappingState {
    Unknown,
    Wating,
    Mapped,
    Failed,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MappingError {
    Badauth,
    Unreachable,
    #[serde(rename = "myplexgone")]
    MyplexGone,
    #[serde(rename = "publisherror")]
    PublishError,
    #[serde(rename = "doublenat")]
    DoubleNat,
    #[serde(rename = "jumboframes")]
    JumboFrames,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MyPlexSignInState {
    Unknown,
    None,
    Invalid,
    Ok,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}
