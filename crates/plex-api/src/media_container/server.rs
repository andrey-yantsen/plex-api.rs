use serde::Deserialize;
use serde_with::{rust::StringWithSeparator, CommaSeparator};
use strum::EnumString;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
    pub country_code: String,
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

#[derive(Debug, Deserialize, Clone, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Diagnostics {
    Logs,
    Databases,
    StreamingLogs,
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[strum(default)]
    UnknownValue(String),
}

// This enum is deserialized in two different ways:
// * From account info, where there's a list of features — `serde` takes care
//   of this
// * From server into, where there's the list of features goes as
//   comma-separated string — `strum` takes care of this
#[derive(Debug, Deserialize, Clone, EnumString, PartialEq, Eq, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Feature {
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
    #[strum(serialize = "photos-metadata-edition")]
    #[serde(rename = "photos-metadata-edition")]
    PhotosMetadataEdition,
    Collections,
    Radio,
    #[strum(serialize = "tuner-sharing")]
    #[serde(rename = "tuner-sharing")]
    TunerSharing,
    #[strum(serialize = "photos-favorites")]
    #[serde(rename = "photos-favorites")]
    PhotosFavorites,
    #[strum(serialize = "hwtranscode")]
    #[serde(rename = "hwtranscode")]
    HardwareTranscode,
    #[strum(serialize = "photosV6-tv-albums")]
    #[serde(rename = "photosV6-tv-albums")]
    PhotosV6TvAlbums,
    #[strum(serialize = "photosV6-edit")]
    #[serde(rename = "photosV6-edit")]
    PhotosV6Edit,
    #[strum(serialize = "federated-auth")]
    #[serde(rename = "federated-auth")]
    FederatedAuth,
    ItemClusters,
    #[strum(serialize = "livetv")]
    #[serde(rename = "livetv")]
    LiveTV,
    #[strum(serialize = "cloud-livetv")]
    #[serde(rename = "cloud-livetv")]
    CloudLiveTV,
    #[strum(serialize = "Android - PiP")]
    #[serde(rename = "Android - PiP")]
    AndroidPictureInPicture,
    #[strum(serialize = "photos-v5")]
    #[serde(rename = "photos-v5")]
    PhotosV5,
    Podcasts,
    #[strum(serialize = "livetv-platform-specific")]
    #[serde(rename = "livetv-platform-specific")]
    LivetvPlatformSpecific,
    #[strum(serialize = "type-first")]
    #[serde(rename = "type-first")]
    TypeFirst,
    #[strum(serialize = "unsupportedtuners")]
    #[serde(rename = "unsupportedtuners")]
    UnsupportedTuners,
    #[strum(serialize = "kevin-bacon")]
    #[serde(rename = "kevin-bacon")]
    KevinBacon,
    #[strum(serialize = "live-tv-channels-grid")]
    #[serde(rename = "live-tv-channels-grid")]
    LiveTvChannelsGrid,
    #[strum(serialize = "Subtitles on Demand")]
    #[serde(rename = "Subtitles on Demand")]
    SubtitlesOnDemand,
    #[strum(serialize = "web-log-viewer")]
    #[serde(rename = "web-log-viewer")]
    WebLogViewer,
    #[strum(serialize = "client-radio-stations")]
    #[serde(rename = "client-radio-stations")]
    ClientRadioStations,
    #[strum(serialize = "imagga-v2")]
    #[serde(rename = "imagga-v2")]
    ImaggaV2,
    #[strum(serialize = "silence-removal")]
    #[serde(rename = "silence-removal")]
    SilenceRemoval,
    #[strum(serialize = "boost-voices")]
    #[serde(rename = "boost-voices")]
    BoostVoices,
    #[strum(serialize = "volume-leveling")]
    #[serde(rename = "volume-leveling")]
    VolumeLeveling,
    #[strum(serialize = "sweet-fades")]
    #[serde(rename = "sweet-fades")]
    SweetFades,
    #[strum(serialize = "sleep-timer")]
    #[serde(rename = "sleep-timer")]
    SleepTimer,
    #[strum(serialize = "TREBLE-show-features")]
    #[serde(rename = "TREBLE-show-features")]
    TrebleShowFeatures,
    WebServerDashboard,
    Visualizers,
    #[strum(serialize = "chromecast-music-mp")]
    #[serde(rename = "chromecast-music-mp")]
    ChromecastMusicMp,
    #[strum(serialize = "premium-dashboard")]
    #[serde(rename = "premium-dashboard")]
    PremiumDashboard,
    ConanRedirectQa,
    ConanRedirectAlpha,
    ConanRedirectBeta,
    ConanRedirectPublic,
    #[strum(serialize = "save-to-library")]
    #[serde(rename = "save-to-library")]
    SaveToLibrary,
    Nominatim,
    TranscoderCache,
    #[strum(serialize = "live-tv-support-incomplete-segments")]
    #[serde(rename = "live-tv-support-incomplete-segments")]
    LiveTvSupportIncompleteSegments,
    #[strum(serialize = "dvr-block-unsupported-countries")]
    #[serde(rename = "dvr-block-unsupported-countries")]
    DvrBlockUnsupportedCountries,
    CompanionsSonos,
    #[strum(serialize = "users-and-sharing")]
    #[serde(rename = "users-and-sharing")]
    UsersAndSharing,
    #[strum(serialize = "web-share-v2")]
    #[serde(rename = "web-share-v2")]
    WebShareV2,
    #[strum(serialize = "sonos-client-feature")]
    #[serde(rename = "sonos-client-feature")]
    SonosClient,
    #[strum(serialize = "artist-tv")]
    #[serde(rename = "artist-tv")]
    ArtistTv,
    AllowDvr,
    MusicPreview,
    SigninNotification,
    #[strum(serialize = "singleitemsharing")]
    #[serde(rename = "singleitemsharing")]
    SingleItemSharing,
    #[strum(serialize = "exclude restrictions")]
    #[serde(rename = "exclude restrictions")]
    ExcludeRestrictions,
    Vod,
    SigninWithApple,
    DrmSupport,
    Metadata,
    #[strum(serialize = "tunefind-clients")]
    #[serde(rename = "tunefind-clients")]
    TunefindClients,
    #[strum(serialize = "Android - Dolby Vision")]
    #[serde(rename = "Android - Dolby Vision")]
    AndroidDolbyVision,
    #[strum(serialize = "parental-controls")]
    #[serde(rename = "parental-controls")]
    ParentalControls,
    #[strum(serialize = "epg-recent-channels")]
    #[serde(rename = "epg-recent-channels")]
    EpgRecentChannels,
    SpringServeAdProvider,
    LetsEncrypt,
    ConanRedirectNightlies,
    ConanRedirectNightly,
    VodSubtitles,
    Watchlist,
    CreateAnonymousUsers,
    #[strum(serialize = "retro-games")]
    #[serde(rename = "retro-games")]
    RetroGames,
    #[strum(serialize = "web-desktop-v4-home")]
    #[serde(rename = "web-desktop-v4-home")]
    WebDesktopV4Home,
    #[strum(serialize = "web-desktop-v4-pre-plays")]
    #[serde(rename = "web-desktop-v4-pre-plays")]
    WebDesktopV4PrePlays,
    #[strum(serialize = "intro-markers")]
    #[serde(rename = "intro-markers")]
    IntroMarkers,
    #[strum(serialize = "client-non-destructive-comskip")]
    #[serde(rename = "client-non-destructive-comskip")]
    ClientNonDestructiveComskip,
    #[strum(serialize = "web-desktop-live-tv-chromecast-remote-player")]
    #[serde(rename = "web-desktop-live-tv-chromecast-remote-player")]
    WebDesktopLiveTvChromecastRemotePlayer,
    #[strum(serialize = "web-desktop-v4-dvr-setup")]
    #[serde(rename = "web-desktop-v4-dvr-setup")]
    WebDesktopV4DvrSetup,
    #[strum(serialize = "watch-together-20200520")]
    #[serde(rename = "watch-together-20200520")]
    WatchTogether20200520,
    #[strum(serialize = "watch-together-invite")]
    #[serde(rename = "watch-together-invite")]
    WatchTogetherInvite,
    #[strum(serialize = "spotlight-style-hub")]
    #[serde(rename = "spotlight-style-hub")]
    SpotlightStyleHub,
    #[strum(serialize = "Sync v3")]
    #[serde(rename = "Sync v3")]
    SyncV3,
    #[strum(serialize = "tunefind-vod")]
    #[serde(rename = "tunefind-vod")]
    TunefindVod,
    #[strum(serialize = "live-tv-on-plex-subtitles")]
    #[serde(rename = "live-tv-on-plex-subtitles")]
    LiveTvOnPlexSubtitles,
    #[strum(serialize = "ios14-privacy-banner")]
    #[serde(rename = "ios14-privacy-banner")]
    Ios14PrivacyBanner,
    #[strum(serialize = "two-factor-authentication")]
    #[serde(rename = "two-factor-authentication")]
    TwoFactorAuthentication,
    #[strum(serialize = "amazon-loop-debug")]
    #[serde(rename = "amazon-loop-debug")]
    AmazonLoopDebug,
    #[strum(serialize = "retro-games-plex-tv")]
    #[serde(rename = "retro-games-plex-tv")]
    RetroGamesPlexTv,
    #[strum(serialize = "two-factor-authentication-clients")]
    #[serde(rename = "two-factor-authentication-clients")]
    TwoFactorAuthenticationClients,
    VodCloudflare,
    #[strum(serialize = "global-continue-watching")]
    #[serde(rename = "global-continue-watching")]
    GlobalContinueWatching,
    #[strum(serialize = "grandfather-sync")]
    #[serde(rename = "grandfather-sync")]
    GrandfatherSync,
    #[strum(serialize = "downloads-gating")]
    #[serde(rename = "downloads-gating")]
    DownloadsGating,
    #[strum(serialize = "optimize-server-users-endpoint")]
    #[serde(rename = "optimize-server-users-endpoint")]
    OptimizeServerUsersEndpoint,
    #[strum(serialize = "web-desktop-gracenote-banner")]
    #[serde(rename = "web-desktop-gracenote-banner")]
    WebDesktopGracenoteBanner,
    MetadataSearch,
    #[strum(serialize = "CU Sunset")]
    #[serde(rename = "CU Sunset")]
    CuSunset,
    #[strum(serialize = "news-provider-sunset-modal")]
    #[serde(rename = "news-provider-sunset-modal")]
    NewsProviderSunsetModal,
    #[strum(serialize = "custom-home-removal")]
    #[serde(rename = "custom-home-removal")]
    CustomHomeRemoval,
    #[strum(serialize = "bypass-web-navbar-upsell-modal")]
    #[serde(rename = "bypass-web-navbar-upsell-modal")]
    BypassWebNavbarUpsellModal,
    #[strum(serialize = "music-analysis")]
    #[serde(rename = "music-analysis")]
    MusicAnalysis,
    #[strum(serialize = "album-types")]
    #[serde(rename = "album-types")]
    AlbumTypes,
    #[strum(serialize = "ad-countdown-timer")]
    #[serde(rename = "ad-countdown-timer")]
    AdCountdownTimer,
    #[strum(serialize = "guided-upgrade")]
    #[serde(rename = "guided-upgrade")]
    GuidedUpgrade,
    #[strum(serialize = "upgrade-3ds2")]
    #[serde(rename = "upgrade-3ds2")]
    Upgrade3ds2,
    DownloadCertificates,
    Loudness,
    #[strum(serialize = "server-manager")]
    #[serde(rename = "server-manager")]
    ServerManager,
    #[strum(serialize = "shared-radio")]
    #[serde(rename = "shared-radio")]
    SharedRadio,
    PhotoAutotags,
    #[strum(
        serialize = "002c9f1a-2fc0-4812-b85b-0e6140f21a0f",
        serialize = "044a1fac-6b55-47d0-9933-25a035709432",
        serialize = "04d7d794-b76c-49ef-9184-52f8f1f501ee",
        serialize = "05690239-443e-43fb-bc1a-95b5d916ca63",
        serialize = "06d14b9e-2af8-4c2b-a4a1-ea9d5c515824",
        serialize = "07f804e6-28e6-4beb-b5c3-f2aefc88b938",
        serialize = "0a348865-4f87-46dc-8bb2-f37637975724",
        serialize = "0de6151c-e0dd-47c8-a81e-1acb977c7f0f",
        serialize = "0eee866d-782b-4dfd-b42b-3bbe8eb0af16",
        serialize = "13056a62-9bd2-47cf-aba9-bab00095fd08",
        serialize = "1417df52-986e-4e4b-8dcd-3997fbc5c976",
        serialize = "16d69c53-4c40-4821-b9f3-57ca690b2d4d",
        serialize = "1844737f-1a87-45c3-ab20-01435959e63c",
        serialize = "1b3a63e4-c2f4-4011-a181-2343d3a97ef7",
        serialize = "1dd846ed-7cde-4dc5-8ef6-53d3ce8c4e9d",
        serialize = "1df3cd16-faf2-4d37-8349-1fcf3713bf1d",
        serialize = "222020fb-1504-492d-af33-a0b80a49558a",
        serialize = "228a6439-ee2f-4a9b-b0fc-1bfcd48b5095",
        serialize = "22b27e12-472e-4383-92ea-2ec3976d8e72",
        serialize = "22d52c96-9e2b-45c0-9e2a-1d6c66ad3474",
        serialize = "24b4cf36-b296-4002-86b7-f1adb657e76a",
        serialize = "2797e341-b062-46ed-862f-0acbba5dd522",
        serialize = "298a11d3-9324-4104-8047-0ac10df4a8a6",
        serialize = "2ea0e464-ea4f-4be2-97c1-ce6ed4b377dd",
        serialize = "300231e0-69aa-4dce-97f4-52d8c00e3e8c",
        serialize = "32cc8bf5-b425-4582-a52d-71b4f1cf436b",
        serialize = "34e182bd-2f62-4678-a9e9-d13b3e25019d",
        serialize = "39dbdd84-8339-4736-96a1-0eb105cc2e08",
        serialize = "3a2b0cb6-1519-4431-98e2-823c248c70eb",
        serialize = "3ae06d3a-a76b-435e-8cef-2d2008610ba2",
        serialize = "3bfd3ccf-8c63-4dbb-8f87-9b21b402c82b",
        serialize = "3c376154-d47e-4bbf-9428-2ea2592fd20a",
        serialize = "4742780c-af9d-4b44-bf5b-7b27e3369aa8",
        serialize = "4b522f91-ae89-4f62-af9c-76f44d8ef61c",
        serialize = "4ca03b04-54c1-4f9f-aea2-f813ae48f317",
        serialize = "4cd4dc0e-6cbe-456c-9988-9f073fadcd73",
        serialize = "4e27cf82-9fb6-4ebe-8e10-c48bfe6fbbb6",
        serialize = "547514ab-3284-46e5-af77-bbaff247e3fc",
        serialize = "55b9f6ed-5d26-4d2d-a436-68882a9901b5",
        serialize = "567033ef-ffee-44fb-8f90-f678077445f9",
        serialize = "5b6190a9-77a4-477e-9fbc-c8118e35a4c1",
        serialize = "5c1951bf-ccf1-4821-8ee7-e50f51218ae7",
        serialize = "5d819d02-5d04-4116-8eec-f49def4e2d6f",
        serialize = "5e2a89ec-fb26-4234-b66e-14d37f35dff2",
        serialize = "62b1e357-5450-41d8-9b60-c7705f750849",
        serialize = "6380e085-02fe-43b5-8bff-380fa4f2423c",
        serialize = "644c4466-05fa-45e0-a478-c594cf81778f",
        serialize = "65152b75-13a9-408a-bd30-dbd23a259183",
        serialize = "65685ff8-4375-4e4c-a806-ec1f0b4a8b7f",
        serialize = "67c80530-eae3-4500-a9fa-9b6947d0f6d1",
        serialize = "68747f3a-ce13-46ce-9274-1e0544c9f500",
        serialize = "6d7be725-9a96-42c7-8af4-01e735138822",
        serialize = "6f82ca43-6117-4e55-ae0e-5ea3b3e99a96",
        serialize = "78643fe5-d192-40c7-8e93-5ccf04c0b767",
        serialize = "7e7596aa-6e2c-41d1-a460-1e13cf0b62f2",
        serialize = "7ee1495c-2798-4288-94e2-9cd98e67d441",
        serialize = "82999dd3-a2be-482e-9f44-357879b4f603",
        serialize = "849433b0-ef60-4a71-9dd9-939bc01f5362",
        serialize = "84a754b0-d1ca-4433-af2d-c949bf4b4936",
        serialize = "850f3d1e-3f38-44c1-9c0c-e3c9127b8b5a",
        serialize = "8536058d-e1dd-4ae7-b30f-e8b059b7cc17",
        serialize = "85ebfb7b-77fb-4afd-bb1a-2fe2fefdddbe",
        serialize = "86da2200-58db-4d78-ba46-f146ba25906b",
        serialize = "88aba3a3-bd62-42a5-91bb-0558a4c1db57",
        serialize = "8e8dd5c8-14a4-4208-97d4-623e09191774",
        serialize = "8fd37970-6e4e-4f00-a64a-e70b52f18e94",
        serialize = "95149521-f64b-46ea-825c-9114e56afd2c",
        serialize = "96cac76e-c5bc-4596-87eb-4fdfef9aaa11",
        serialize = "98872b06-2ff3-4b71-96bc-039e2ebe7adc",
        serialize = "9a67bff2-cb80-4bf9-81c6-9ad2f4c78afd",
        serialize = "9c982beb-c676-4d6f-a777-ff5d37ec3081",
        serialize = "9dc1df45-fb45-4be1-9ab2-eb23eb57f082",
        serialize = "a19d495a-1cef-4f7c-ab77-5186e63e17f7",
        serialize = "a3d2d5c4-46a0-436e-a2d6-80d26f32b369",
        serialize = "a4bc568b-477f-4f36-894b-49e19f34353f",
        serialize = "a536a6e1-0ece-498a-bf64-99b53c27de3a",
        serialize = "a548af72-b804-4d05-8569-52785952d31d",
        serialize = "a6e0a154-4735-4cbb-a6ec-7a0a146c8216",
        serialize = "a6f3f9b3-c10c-4b94-ad59-755e30ac6c90",
        serialize = "abd37b14-706c-461f-8255-fa9563882af3",
        serialize = "b20d91ca-1b2f-45a2-a115-c1ad24c66ac5",
        serialize = "b227c158-e062-4ff1-95d8-8ed11cecafb1",
        serialize = "b2403ac6-4885-4971-8b96-59353fd87c72",
        serialize = "b46d16ae-cbd6-4226-8ee9-ab2b27e5dd42",
        serialize = "b5874ecb-6610-47b2-8906-1b5a897acb02",
        serialize = "b58d7f28-7b4a-49bb-97a7-152645505f28",
        serialize = "b612f571-83c3-431a-88eb-3f05ce08da4a",
        serialize = "b77e6744-c18d-415a-8e7c-7aac5d7a7750",
        serialize = "b83c8dc9-5a01-4b7a-a7c9-5870c8a6e21b",
        serialize = "b8cf9f40-4f8a-4de4-b203-5bbcf8b09f5a",
        serialize = "bb50c92f-b412-44fe-8d8a-b1684f212a44",
        serialize = "bbf73498-4912-4d80-9560-47c4fe212cec",
        serialize = "bc8d1fca-deb0-4d0a-a6f4-12cfd681002d",
        serialize = "bfeaee4e-965a-4d24-b163-020c3c57d936",
        serialize = "c2409baa-d044-45c7-b1f4-e9e7ccd2d128",
        serialize = "c55d5900-b546-416d-a8c5-45b24a13e9bc",
        serialize = "c5adf9dc-af13-4a85-a24b-98de6fa2f595",
        serialize = "c7ae6f8f-05e6-48bb-9024-c05c1dc3c43e",
        serialize = "c92d4903-bc06-4715-8ce4-4a22674abac8",
        serialize = "c9d9b7ee-fdd9-474e-b143-5039c04e9b9b",
        serialize = "cc9bea3b-11ab-4402-a222-4958bb129cab",
        serialize = "d14556be-ae6d-4407-89d0-b83953f4789a",
        serialize = "d1477307-4dac-4e57-9258-252e5b908693",
        serialize = "d20f9af2-fdb1-4927-99eb-a2eb8fbff799",
        serialize = "d413fb56-de7b-40e4-acd0-f3dbb7c9e104",
        serialize = "d85cb60c-0986-4a02-b1e1-36c64c609712",
        serialize = "d8810b38-ec9b-494c-8555-3df6e365dfbd",
        serialize = "d9f42aea-bc9d-47db-9814-cd7a577aff48",
        serialize = "dab501df-5d99-48ef-afc2-3e839e4ddc9a",
        serialize = "db965785-ca5c-46fd-bab6-7b3d29c18492",
        serialize = "ddd730e1-a0a0-429f-a7d3-7c5001d24497",
        serialize = "e45bc5ae-1c3a-4729-922b-c69388c571b7",
        serialize = "e66aa31c-abdd-483d-93bc-e17485d8837f",
        serialize = "e8230c74-0940-4b91-9e20-6571eb068086",
        serialize = "e954ef21-08b4-411e-a1f0-7551f1e57b11",
        serialize = "ea442c16-044a-4fa7-8461-62643f313c62",
        serialize = "ec64b6f6-e804-4ef3-b114-9d5c63e1a941",
        serialize = "ee352392-2934-4061-ba35-5f3189f19ab4",
        serialize = "f3235e61-c0eb-4718-ac0a-7d6eb3d8ff75",
        serialize = "f3a99481-9671-4274-a0d3-4c06a72ef746",
        serialize = "f83450e2-759a-4de4-8b31-e4a163896d43",
        serialize = "f87f382b-4a41-4951-b4e4-d5822c69e4c6",
        serialize = "f8ea4f37-c554-476a-8852-1cbd2912f3f6",
        serialize = "fb34e64d-cd89-47b8-8bae-a6d20c542bae",
        serialize = "fd6683b9-1426-4b00-840f-cd5fb0904a6a",
        serialize = "fec722a0-a6d4-4fbd-96dc-4ffb02b072c5",
        serialize = "e7cea823-02e5-48c4-a501-d37b82bf132f"
    )]
    StrangeUuid,
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[strum(default)]
    UnknownValue(String),
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue2,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MappingState {
    Unknown,
    Wating,
    Mapped,
    Failed,
    #[cfg(not(feature = "deny_unknown_fields"))]
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
    #[cfg(not(feature = "deny_unknown_fields"))]
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
    #[cfg(not(feature = "deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}
