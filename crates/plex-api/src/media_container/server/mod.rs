mod feature;
pub mod library;

pub use self::feature::Feature;
use self::library::ContentDirectory;
use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

#[derive(Debug, Deserialize, Clone)]
pub struct Action {
    pub id: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MediaProviderFeature {
    Content {
        key: String,
        #[serde(rename = "Directory")]
        directory: Vec<ContentDirectory>,
    },
    Search {
        key: String,
    },
    Match {
        key: String,
    },
    Metadata {
        key: String,
    },
    Rate {
        key: String,
    },
    ImageTranscoder {
        key: String,
    },
    Promoted {
        key: String,
    },
    ContinueWatching {
        key: String,
    },
    Actions {
        key: String,
        #[serde(rename = "Action")]
        actions: Vec<Action>,
    },
    Playlist {
        key: String,
        flavor: String,
    },
    PlayQueue {
        key: String,
        flavor: String,
    },
    #[serde(rename = "collection")]
    Collections {
        key: String,
    },
    #[serde(rename_all = "camelCase")]
    Timeline {
        key: String,
        scrobble_key: String,
        unscrobble_key: String,
    },
    Manage,
    #[serde(rename = "queryParser")]
    QueryParser,
    Subscribe {
        flavor: String,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MediaProviderProtocol {
    Stream,
    Download,
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(MediaProviderProtocol);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MediaProviderType {
    Video,
    Audio,
    Photo,
}

derive_fromstr_from_deserialize!(MediaProviderType);

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaProvider {
    pub identifier: String,
    pub id: Option<u32>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, MediaProviderProtocol>")]
    pub protocols: Vec<MediaProviderProtocol>,
    pub title: String,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, MediaProviderType>")]
    pub types: Vec<MediaProviderType>,
    #[serde(rename = "Feature")]
    pub features: Vec<MediaProviderFeature>,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u32>,
    pub provider_identifier: Option<String>,
    pub epg_source: Option<String>,
    pub friendly_name: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub size: u32,
    pub allow_camera_upload: bool,
    pub allow_channel_access: bool,
    pub allow_media_deletion: Option<bool>,
    pub allow_sharing: bool,
    pub allow_sync: bool,
    pub allow_tuners: bool,
    pub background_processing: bool,
    pub certificate: Option<bool>,
    pub companion_proxy: bool,
    pub country_code: Option<String>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Diagnostics>")]
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
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Feature>")]
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
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, u32>")]
    pub transcoder_video_bitrates: Vec<u32>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, u8>")]
    pub transcoder_video_qualities: Vec<u8>,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, u16>")]
    pub transcoder_video_resolutions: Vec<u16>,
    pub updated_at: i64,
    pub updater: bool,
    pub version: String,
    pub voice_search: bool,
    #[serde(rename = "MediaProvider")]
    pub media_providers: Vec<MediaProvider>,
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
