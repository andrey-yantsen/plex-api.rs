use serde::Deserialize;
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;
use std::fmt;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSummary {
    pub active: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub subscribed_at: Option<OffsetDateTime>,
    pub status: SubscriptionStatus,
    pub payment_service: Option<String>,
    pub plan: Option<String>,
    pub features: Vec<SubscriptionFeature>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Canceled,
    PendingCancellation,
    Ended,
    Lapsed,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionFeature {
    Webhooks,
    CameraUpload,
    Home,
    Pass,
    Dvr,
    Trailers,
    SessionBandwidthRestrictions,
    MusicVideos,
    ContentFilter,
    AdaptiveBitrate,
    Sync,
    Lyrics,
    Cloudsync,
    PremiumMusicMetadata,
    HardwareTranscoding,
    SessionKick,
    Music,
    #[serde(rename = "photos-metadata-edition")]
    PhotosMetadataEdition,
    Collections,
    Radio,
    #[serde(rename = "tuner-sharing")]
    TunerSharing,
    #[serde(rename = "photos-favorites")]
    PhotosFavorites,
    #[serde(rename = "hwtranscode")]
    HardwareTranscode,
    #[serde(rename = "photosV6-tv-albums")]
    PhotosV6TvAlbums,
    #[serde(rename = "photosV6-edit")]
    PhotosV6Edit,
    #[serde(rename = "federated-auth")]
    FederatedAuth,
    ItemClusters,
    #[serde(rename = "livetv")]
    LiveTV,
    #[serde(rename = "cloud-livetv")]
    CloudLiveTV,
    #[serde(rename = "Android - PiP")]
    AndroidPictureInPicture,
    #[serde(rename = "photos-v5")]
    PhotosV5,
    Podcasts,
    #[serde(rename = "livetv-platform-specific")]
    LivetvPlatformSpecific,
    #[serde(rename = "type-first")]
    TypeFirst,
    #[serde(rename = "unsupportedtuners")]
    UnsupportedTuners,
    #[serde(rename = "kevin-bacon")]
    KevinBacon,
    #[serde(rename = "live-tv-channels-grid")]
    LiveTvChannelsGrid,
    #[serde(rename = "Subtitles on Demand")]
    SubtitlesOnDemand,
    #[serde(rename = "web-log-viewer")]
    WebLogViewer,
    #[serde(rename = "client-radio-stations")]
    ClientRadioStations,
    #[serde(rename = "imagga-v2")]
    ImaggaV2,
    #[serde(rename = "silence-removal")]
    SilenceRemoval,
    #[serde(rename = "boost-voices")]
    BoostVoices,
    #[serde(rename = "volume-leveling")]
    VolumeLeveling,
    #[serde(rename = "sweet-fades")]
    SweetFades,
    #[serde(rename = "sleep-timer")]
    SleepTimer,
    #[serde(rename = "TREBLE-show-features")]
    TrebleShowFeatures,
    WebServerDashboard,
    Visualizers,
    #[serde(rename = "chromecast-music-mp")]
    ChromecastMusicMp,
    #[serde(rename = "premium-dashboard")]
    PremiumDashboard,
    ConanRedirectQa,
    ConanRedirectAlpha,
    ConanRedirectBeta,
    ConanRedirectPublic,
    #[serde(rename = "save-to-library")]
    SaveToLibrary,
    Nominatim,
    TranscoderCache,
    #[serde(rename = "live-tv-support-incomplete-segments")]
    LiveTvSupportIncompleteSegments,
    #[serde(rename = "dvr-block-unsupported-countries")]
    DvrBlockUnsupportedCountries,
    CompanionsSonos,
    #[serde(rename = "users-and-sharing")]
    UsersAndSharing,
    #[serde(rename = "web-share-v2")]
    WebShareV2,
    #[serde(rename = "sonos-client-feature")]
    SonosClientFeature,
    #[serde(rename = "artist-tv")]
    ArtistTv,
    AllowDvr,
    MusicPreview,
    SigninNotification,
    #[serde(rename = "singleitemsharing")]
    SingleItemSharing,
    #[serde(rename = "exclude restrictions")]
    ExcludeRestrictions,
    Vod,
    SigninWithApple,
    DrmSupport,
    Metadata,
    #[serde(rename = "tunefind-clients")]
    TunefindClients,
    #[serde(rename = "Android - Dolby Vision")]
    AndroidDolbyVision,
    #[serde(rename = "parental-controls")]
    ParentalControls,
    #[serde(rename = "epg-recent-channels")]
    EpgRecentChannels,
    SpringServeAdProvider,
    LetsEncrypt,
    ConanRedirectNightlies,
    ConanRedirectNightly,
    VodSubtitles,
    Watchlist,
    CreateAnonymousUsers,
    #[serde(rename = "retro-games")]
    RetroGames,
    #[serde(rename = "web-desktop-v4-home")]
    WebDesktopV4Home,
    #[serde(rename = "web-desktop-v4-pre-plays")]
    WebDesktopV4PrePlays,
    #[serde(rename = "intro-markers")]
    IntroMarkers,
    #[serde(rename = "client-non-destructive-comskip")]
    ClientNonDestructiveComskip,
    #[serde(rename = "web-desktop-live-tv-chromecast-remote-player")]
    WebDesktopLiveTvChromecastRemotePlayer,
    #[serde(rename = "web-desktop-v4-dvr-setup")]
    WebDesktopV4DvrSetup,
    #[serde(rename = "watch-together-20200520")]
    WatchTogether20200520,
    #[serde(rename = "watch-together-invite")]
    WatchTogetherInvite,
    #[serde(rename = "spotlight-style-hub")]
    SpotlightStyleHub,
    #[serde(rename = "Sync v3")]
    SyncV3,
    #[serde(rename = "tunefind-vod")]
    TunefindVod,
    #[serde(rename = "live-tv-on-plex-subtitles")]
    LiveTvOnPlexSubtitles,
    #[serde(rename = "ios14-privacy-banner")]
    Ios14PrivacyBanner,
    #[serde(rename = "two-factor-authentication")]
    TwoFactorAuthentication,
    #[serde(rename = "amazon-loop-debug")]
    AmazonLoopDebug,
    #[serde(rename = "retro-games-plex-tv")]
    RetroGamesPlexTv,
    #[serde(rename = "two-factor-authentication-clients")]
    TwoFactorAuthenticationClients,
    VodCloudflare,
    #[serde(rename = "global-continue-watching")]
    GlobalContinueWatching,
    #[serde(rename = "grandfather-sync")]
    GrandfatherSync,
    #[serde(rename = "downloads-gating")]
    DownloadsGating,
    #[serde(rename = "optimize-server-users-endpoint")]
    OptimizeServerUsersEndpoint,
    #[serde(rename = "web-desktop-gracenote-banner")]
    WebDesktopGracenoteBanner,
    MetadataSearch,
    #[serde(rename = "CU Sunset")]
    CuSunset,
    #[serde(rename = "news-provider-sunset-modal")]
    NewsProviderSunsetModal,
    #[serde(rename = "custom-home-removal")]
    CustomHomeRemoval,
    #[serde(rename = "bypass-web-navbar-upsell-modal")]
    BypassWebNavbarUpsellModal,
    #[serde(rename = "music-analysis")]
    MusicAnalysis,
    #[serde(rename = "album-types")]
    AlbumTypes,
    #[serde(rename = "ad-countdown-timer")]
    AdCountdownTimer,
    #[serde(rename = "guided-upgrade")]
    GuidedUpgrade,
    #[serde(rename = "upgrade-3ds2")]
    Upgrade3ds2,
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

impl fmt::Display for SubscriptionFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum AutoSelectSubtitleMode {
    ManuallySelected = 0,
    ShownWithForeignAudio = 1,
    AlwaysEnabled = 2,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub auto_select_audio: bool,
    pub auto_select_subtitle: AutoSelectSubtitleMode,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub default_subtitle_accessibility: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub default_subtitle_forced: bool,
    pub default_audio_language: Option<String>,
    pub default_subtitle_language: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,
    pub mode: String,
    pub state: String,
    pub renews_at: Option<OffsetDateTime>,
    pub ends_at: Option<OffsetDateTime>,
    pub r#type: Option<String>,
    pub transfer: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Service {
    pub identifier: String,
    pub endpoint: String,
    pub token: Option<String>,
    pub secret: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MyPlexAccount {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub friendly_name: String,
    pub confirmed: bool,
    pub title: String,
    pub email: String,
    pub thumb: String,
    pub locale: Option<String>,
    pub email_only_auth: bool,
    pub has_password: bool,
    pub cloud_sync_device: Option<String>,
    pub auth_token: String,
    pub mailing_list_status: Option<String>,
    pub mailing_list_active: bool,
    pub scrobble_types: String,
    pub pin: Option<String>,
    pub subscription: SubscriptionSummary,
    pub subscription_description: Option<String>,
    pub restricted: bool,
    pub home: bool,
    pub guest: bool,
    pub queue_email: Option<String>,
    pub queue_uid: Option<HashMap<String, String>>,
    pub home_size: i32,
    pub max_home_size: i32,
    pub certificate_version: i32,
    #[serde(with = "time::serde::timestamp::option")]
    pub remember_expires_at: Option<OffsetDateTime>,
    pub profile: Profile,
    pub entitlements: Vec<String>,
    pub roles: Option<Vec<String>>,
    pub services: Vec<Service>,
    pub protected: bool,
    pub country: String,
    pub home_admin: bool,
    pub ads_consent: Option<bool>,
    #[serde(with = "time::serde::timestamp::option")]
    pub ads_consent_set_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    pub ads_consent_reminder_at: Option<OffsetDateTime>,
    pub anonymous: Option<bool>,
    pub experimental_features: bool,
    pub two_factor_enabled: bool,
    pub backup_codes_created: bool,

    pub restriction_profile: Option<RestrictionProfile>,

    // Some managed guest users don't have subscription info
    pub subscriptions: Option<Vec<Subscription>>,
    pub past_subscriptions: Option<Vec<Subscription>>,
    pub trials: Option<Vec<Subscription>>,

    // TODO: The next field was met only once
    pub settings: Option<Vec<Settings>>,

    // TODO: I don't have a valid example with the data yet for the following fields
    pub custom_restrictions: Option<CustomRestrictions>,
    pub providers: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct CustomRestrictions {
    pub all: Option<bool>,
    pub movies: Option<bool>,
    pub music: Option<bool>,
    pub photos: Option<bool>,
    pub television: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "id", rename_all = "camelCase")]
pub enum Settings {
    Experience(ExperienceSettingsContainer),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSettingsContainer {
    pub hidden: bool,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,

    #[serde(flatten)]
    pub settings: ExperienceSettingsFormat,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum ExperienceSettingsFormat {
    #[serde(with = "serde_with::json::nested")]
    Json(ExperienceSettings),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSettings {
    pub auto_home_hubs_enabled: bool,
    pub auto_pinned_providers: Vec<String>,
    pub schema_version: i32,
    pub home_settings: ExperienceHomeSettings,
    pub sidebar_settings: ExperienceSidebarSettings,
    pub reminders: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceHomeSettings {
    pub settings_key: String,
    pub hubs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSidebarSettings {
    pub has_completed_setup: bool,
    pub pinned_sources: Vec<SidebarSource>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SidebarSource {
    pub key: String,
    pub source_type: String,
    pub machine_identifier: String,
    pub provider_identifier: String,
    #[serde(rename = "directoryID")]
    pub directory_id: String,
    pub directory_icon: String,
    pub title: String,
    pub server_friendly_name: String,
    pub provider_source_title: String,
    pub is_cloud: bool,
    pub is_full_owned_server: bool,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionProfile {
    LittleKid,
    OlderKid,
    Teen,
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}
