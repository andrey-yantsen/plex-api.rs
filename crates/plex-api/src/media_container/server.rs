use serde::{Deserialize, Serialize};
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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
    SonosClient,
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
    DownloadCertificates,
    Loudness,
    #[serde(rename = "server-manager")]
    ServerManager,
    #[serde(rename = "shared-radio")]
    SharedRadio,
    PhotoAutotags,
    #[serde(
        alias = "002c9f1a-2fc0-4812-b85b-0e6140f21a0f",
        alias = "044a1fac-6b55-47d0-9933-25a035709432",
        alias = "04d7d794-b76c-49ef-9184-52f8f1f501ee",
        alias = "05690239-443e-43fb-bc1a-95b5d916ca63",
        alias = "06d14b9e-2af8-4c2b-a4a1-ea9d5c515824",
        alias = "07f804e6-28e6-4beb-b5c3-f2aefc88b938",
        alias = "0a348865-4f87-46dc-8bb2-f37637975724",
        alias = "0de6151c-e0dd-47c8-a81e-1acb977c7f0f",
        alias = "0eee866d-782b-4dfd-b42b-3bbe8eb0af16",
        alias = "13056a62-9bd2-47cf-aba9-bab00095fd08",
        alias = "1417df52-986e-4e4b-8dcd-3997fbc5c976",
        alias = "16d69c53-4c40-4821-b9f3-57ca690b2d4d",
        alias = "1844737f-1a87-45c3-ab20-01435959e63c",
        alias = "1b3a63e4-c2f4-4011-a181-2343d3a97ef7",
        alias = "1dd846ed-7cde-4dc5-8ef6-53d3ce8c4e9d",
        alias = "1df3cd16-faf2-4d37-8349-1fcf3713bf1d",
        alias = "222020fb-1504-492d-af33-a0b80a49558a",
        alias = "228a6439-ee2f-4a9b-b0fc-1bfcd48b5095",
        alias = "22b27e12-472e-4383-92ea-2ec3976d8e72",
        alias = "22d52c96-9e2b-45c0-9e2a-1d6c66ad3474",
        alias = "24b4cf36-b296-4002-86b7-f1adb657e76a",
        alias = "2797e341-b062-46ed-862f-0acbba5dd522",
        alias = "298a11d3-9324-4104-8047-0ac10df4a8a6",
        alias = "2ea0e464-ea4f-4be2-97c1-ce6ed4b377dd",
        alias = "300231e0-69aa-4dce-97f4-52d8c00e3e8c",
        alias = "32cc8bf5-b425-4582-a52d-71b4f1cf436b",
        alias = "34e182bd-2f62-4678-a9e9-d13b3e25019d",
        alias = "39dbdd84-8339-4736-96a1-0eb105cc2e08",
        alias = "3a2b0cb6-1519-4431-98e2-823c248c70eb",
        alias = "3ae06d3a-a76b-435e-8cef-2d2008610ba2",
        alias = "3bfd3ccf-8c63-4dbb-8f87-9b21b402c82b",
        alias = "3c376154-d47e-4bbf-9428-2ea2592fd20a",
        alias = "4742780c-af9d-4b44-bf5b-7b27e3369aa8",
        alias = "4b522f91-ae89-4f62-af9c-76f44d8ef61c",
        alias = "4ca03b04-54c1-4f9f-aea2-f813ae48f317",
        alias = "4cd4dc0e-6cbe-456c-9988-9f073fadcd73",
        alias = "4e27cf82-9fb6-4ebe-8e10-c48bfe6fbbb6",
        alias = "547514ab-3284-46e5-af77-bbaff247e3fc",
        alias = "55b9f6ed-5d26-4d2d-a436-68882a9901b5",
        alias = "567033ef-ffee-44fb-8f90-f678077445f9",
        alias = "5b6190a9-77a4-477e-9fbc-c8118e35a4c1",
        alias = "5c1951bf-ccf1-4821-8ee7-e50f51218ae7",
        alias = "5d819d02-5d04-4116-8eec-f49def4e2d6f",
        alias = "5e2a89ec-fb26-4234-b66e-14d37f35dff2",
        alias = "62b1e357-5450-41d8-9b60-c7705f750849",
        alias = "6380e085-02fe-43b5-8bff-380fa4f2423c",
        alias = "644c4466-05fa-45e0-a478-c594cf81778f",
        alias = "65152b75-13a9-408a-bd30-dbd23a259183",
        alias = "65685ff8-4375-4e4c-a806-ec1f0b4a8b7f",
        alias = "67c80530-eae3-4500-a9fa-9b6947d0f6d1",
        alias = "68747f3a-ce13-46ce-9274-1e0544c9f500",
        alias = "6d7be725-9a96-42c7-8af4-01e735138822",
        alias = "6f82ca43-6117-4e55-ae0e-5ea3b3e99a96",
        alias = "78643fe5-d192-40c7-8e93-5ccf04c0b767",
        alias = "7e7596aa-6e2c-41d1-a460-1e13cf0b62f2",
        alias = "7ee1495c-2798-4288-94e2-9cd98e67d441",
        alias = "82999dd3-a2be-482e-9f44-357879b4f603",
        alias = "849433b0-ef60-4a71-9dd9-939bc01f5362",
        alias = "84a754b0-d1ca-4433-af2d-c949bf4b4936",
        alias = "850f3d1e-3f38-44c1-9c0c-e3c9127b8b5a",
        alias = "8536058d-e1dd-4ae7-b30f-e8b059b7cc17",
        alias = "85ebfb7b-77fb-4afd-bb1a-2fe2fefdddbe",
        alias = "86da2200-58db-4d78-ba46-f146ba25906b",
        alias = "88aba3a3-bd62-42a5-91bb-0558a4c1db57",
        alias = "8e8dd5c8-14a4-4208-97d4-623e09191774",
        alias = "8fd37970-6e4e-4f00-a64a-e70b52f18e94",
        alias = "95149521-f64b-46ea-825c-9114e56afd2c",
        alias = "96cac76e-c5bc-4596-87eb-4fdfef9aaa11",
        alias = "98872b06-2ff3-4b71-96bc-039e2ebe7adc",
        alias = "9a67bff2-cb80-4bf9-81c6-9ad2f4c78afd",
        alias = "9c982beb-c676-4d6f-a777-ff5d37ec3081",
        alias = "9dc1df45-fb45-4be1-9ab2-eb23eb57f082",
        alias = "a19d495a-1cef-4f7c-ab77-5186e63e17f7",
        alias = "a3d2d5c4-46a0-436e-a2d6-80d26f32b369",
        alias = "a4bc568b-477f-4f36-894b-49e19f34353f",
        alias = "a536a6e1-0ece-498a-bf64-99b53c27de3a",
        alias = "a548af72-b804-4d05-8569-52785952d31d",
        alias = "a6e0a154-4735-4cbb-a6ec-7a0a146c8216",
        alias = "a6f3f9b3-c10c-4b94-ad59-755e30ac6c90",
        alias = "abd37b14-706c-461f-8255-fa9563882af3",
        alias = "b20d91ca-1b2f-45a2-a115-c1ad24c66ac5",
        alias = "b227c158-e062-4ff1-95d8-8ed11cecafb1",
        alias = "b2403ac6-4885-4971-8b96-59353fd87c72",
        alias = "b46d16ae-cbd6-4226-8ee9-ab2b27e5dd42",
        alias = "b5874ecb-6610-47b2-8906-1b5a897acb02",
        alias = "b58d7f28-7b4a-49bb-97a7-152645505f28",
        alias = "b612f571-83c3-431a-88eb-3f05ce08da4a",
        alias = "b77e6744-c18d-415a-8e7c-7aac5d7a7750",
        alias = "b83c8dc9-5a01-4b7a-a7c9-5870c8a6e21b",
        alias = "b8cf9f40-4f8a-4de4-b203-5bbcf8b09f5a",
        alias = "bb50c92f-b412-44fe-8d8a-b1684f212a44",
        alias = "bbf73498-4912-4d80-9560-47c4fe212cec",
        alias = "bc8d1fca-deb0-4d0a-a6f4-12cfd681002d",
        alias = "bfeaee4e-965a-4d24-b163-020c3c57d936",
        alias = "c2409baa-d044-45c7-b1f4-e9e7ccd2d128",
        alias = "c55d5900-b546-416d-a8c5-45b24a13e9bc",
        alias = "c5adf9dc-af13-4a85-a24b-98de6fa2f595",
        alias = "c7ae6f8f-05e6-48bb-9024-c05c1dc3c43e",
        alias = "c92d4903-bc06-4715-8ce4-4a22674abac8",
        alias = "c9d9b7ee-fdd9-474e-b143-5039c04e9b9b",
        alias = "cc9bea3b-11ab-4402-a222-4958bb129cab",
        alias = "d14556be-ae6d-4407-89d0-b83953f4789a",
        alias = "d1477307-4dac-4e57-9258-252e5b908693",
        alias = "d20f9af2-fdb1-4927-99eb-a2eb8fbff799",
        alias = "d413fb56-de7b-40e4-acd0-f3dbb7c9e104",
        alias = "d85cb60c-0986-4a02-b1e1-36c64c609712",
        alias = "d8810b38-ec9b-494c-8555-3df6e365dfbd",
        alias = "d9f42aea-bc9d-47db-9814-cd7a577aff48",
        alias = "dab501df-5d99-48ef-afc2-3e839e4ddc9a",
        alias = "db965785-ca5c-46fd-bab6-7b3d29c18492",
        alias = "ddd730e1-a0a0-429f-a7d3-7c5001d24497",
        alias = "e45bc5ae-1c3a-4729-922b-c69388c571b7",
        alias = "e66aa31c-abdd-483d-93bc-e17485d8837f",
        alias = "e8230c74-0940-4b91-9e20-6571eb068086",
        alias = "e954ef21-08b4-411e-a1f0-7551f1e57b11",
        alias = "ea442c16-044a-4fa7-8461-62643f313c62",
        alias = "ec64b6f6-e804-4ef3-b114-9d5c63e1a941",
        alias = "ee352392-2934-4061-ba35-5f3189f19ab4",
        alias = "f3235e61-c0eb-4718-ac0a-7d6eb3d8ff75",
        alias = "f3a99481-9671-4274-a0d3-4c06a72ef746",
        alias = "f83450e2-759a-4de4-8b31-e4a163896d43",
        alias = "f87f382b-4a41-4951-b4e4-d5822c69e4c6",
        alias = "f8ea4f37-c554-476a-8852-1cbd2912f3f6",
        alias = "fb34e64d-cd89-47b8-8bae-a6d20c542bae",
        alias = "fd6683b9-1426-4b00-840f-cd5fb0904a6a",
        alias = "fec722a0-a6d4-4fbd-96dc-4ffb02b072c5",
        alias = "e7cea823-02e5-48c4-a501-d37b82bf132f"
    )]
    StrangeUuid,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

derive_fromstr_from_deserialize!(Feature);
derive_display_from_serialize!(Feature);

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
