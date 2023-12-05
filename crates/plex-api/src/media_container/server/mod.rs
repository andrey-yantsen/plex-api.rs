mod feature;
pub mod library;

pub use self::feature::Feature;
use self::library::ContentDirectory;
use crate::media_container::helpers::StringWithSeparatorOrList;
use serde::Deserialize;
use serde_plain::derive_fromstr_from_deserialize;
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,
    pub key: String,
    pub reverse_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct GridChannelFilter {
    pub key: String,
    pub title: String,
    pub genre_rating_key: String,
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
    #[serde(rename = "universalsearch")]
    UniversalSearch {
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
        public: Option<bool>,
    },
    Promoted {
        key: String,
    },
    ContinueWatching {
        key: String,
    },
    Availability {
        key: String,
    },
    #[serde(rename = "availability-platforms")]
    AvailabilityPlatforms {
        key: String,
    },
    Actions {
        key: Option<String>,
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
        scrobble_key: Option<String>,
        unscrobble_key: Option<String>,
    },
    Manage,
    #[serde(rename = "queryParser")]
    QueryParser,
    Subscribe {
        flavor: String,
    },
    Grid {
        key: String,
        #[serde(rename = "GridChannelFilter")]
        grid_channel_filter: Vec<GridChannelFilter>,
    },
    Settings {
        key: String,
    },
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MediaProviderProtocol {
    Stream,
    Download,
    #[serde(rename = "livetv")]
    LiveTv,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(MediaProviderProtocol);

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MediaProviderType {
    Video,
    Audio,
    Photo,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(MediaProviderType);

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MediaProviderWrapper {
    pub(crate) media_provider: MediaProvider,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaProvider {
    pub identifier: String,
    pub id: Option<u32>,
    #[serde_as(as = "StringWithSeparatorOrList::<CommaSeparator, MediaProviderProtocol>")]
    pub protocols: Vec<MediaProviderProtocol>,
    pub title: String,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, MediaProviderType>")]
    #[serde(default)]
    pub types: Vec<MediaProviderType>,
    #[serde(rename = "Feature")]
    pub features: Vec<MediaProviderFeature>,
    #[serde(rename = "parentID")]
    pub parent_id: Option<u32>,
    pub provider_identifier: Option<String>,
    pub epg_source: Option<String>,
    pub friendly_name: Option<String>,
    pub icon: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StartState {
    StartingPlugins,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Other,
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
    pub photo_auto_tag: Option<bool>,
    pub platform: String,
    pub platform_version: String,
    pub plugin_host: bool,
    pub push_notifications: bool,
    pub read_only_libraries: bool,
    pub start_state: Option<StartState>,
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
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
    pub updater: bool,
    pub version: String,
    pub voice_search: bool,
    #[serde(rename = "MediaProvider")]
    pub media_providers: Vec<MediaProvider>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
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

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MappingState {
    Unknown,
    Waiting,
    Mapped,
    Failed,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

#[derive(Debug, Deserialize, Clone, Copy)]
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

#[derive(Debug, Deserialize, Clone, Copy)]
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
