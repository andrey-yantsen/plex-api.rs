#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
#[rustfmt::skip]
pub enum Feature {
    #[serde(
        rename = "ad-countdown-timer",
        alias = "3ae06d3a-a76b-435e-8cef-2d2008610ba2"
    )]
    AdCountdownTimer,
    #[serde(
        rename = "adaptive_bitrate",
        alias = "abd37b14-706c-461f-8255-fa9563882af3"
    )]
    AdaptiveBitrate,
    #[serde(
        rename = "album-types",
        alias = "1df3cd16-faf2-4d37-8349-1fcf3713bf1d"
    )]
    AlbumTypes,
    #[serde(
        rename = "allow_dvr",
        alias = "d8810b38-ec9b-494c-8555-3df6e365dfbd"
    )]
    AllowDvr,
    #[serde(
        rename = "amazon-loop-debug",
        alias = "d1477307-4dac-4e57-9258-252e5b908693"
    )]
    AmazonLoopDebug,
    #[serde(
        rename = "Android - Dolby Vision",
        alias = "b227c158-e062-4ff1-95d8-8ed11cecafb1"
    )]
    AndroidDolbyVision,
    #[serde(
        rename = "Android - PiP",
        alias = "86da2200-58db-4d78-ba46-f146ba25906b"
    )]
    AndroidPip,
    #[serde(
        rename = "artist-tv",
        alias = "2797e341-b062-46ed-862f-0acbba5dd522"
    )]
    ArtistTv,
    #[serde(
        rename = "boost-voices",
        alias = "c2409baa-d044-45c7-b1f4-e9e7ccd2d128"
    )]
    BoostVoices,
    #[serde(
        rename = "bypass-web-navbar-upsell-modal",
        alias = "6aaaf4fc-c55f-4042-92c8-b35f7886d249"
    )]
    BypassWebNavbarUpsellModal,
    #[serde(
        rename = "camera_upload",
        alias = "fb34e64d-cd89-47b8-8bae-a6d20c542bae"
    )]
    CameraUpload,
    #[serde(
        rename = "chromecast-music-mp",
        alias = "58829fc9-26b8-41f4-a6c0-90ea7a11ae24"
    )]
    ChromecastMusicMp,
    #[serde(
        rename = "client-non-destructive-comskip",
        alias = "6ab6677b-ad9b-444f-9ca1-b8027d05b3e1"
    )]
    ClientNonDestructiveComskip,
    #[serde(
        rename = "client-radio-stations",
        alias = "5b6190a9-77a4-477e-9fbc-c8118e35a4c1"
    )]
    ClientRadioStations,
    #[serde(
        rename = "cloud-livetv",
        alias = "dbab9396-78ff-48f5-a5ce-c76539ed1b6e"
    )]
    CloudLivetv,
    #[serde(
        rename = "cloudsync",
        alias = "65152b75-13a9-408a-bd30-dbd23a259183"
    )]
    Cloudsync,
    #[serde(
        rename = "collections",
        alias = "1417df52-986e-4e4b-8dcd-3997fbc5c976"
    )]
    Collections,
    #[serde(
        rename = "community-phase0",
        alias = "fc3e8322-5e6e-4f4a-9d71-728c6d5656bd"
    )]
    CommunityPhase0,
    #[serde(
        rename = "community-phase0-web",
        alias = "a852775a-2b74-4624-aaa3-3d624471a537"
    )]
    CommunityPhase0Web,
    #[serde(
        rename = "companions_sonos",
        alias = "24b4cf36-b296-4002-86b7-f1adb657e76a"
    )]
    CompanionsSonos,
    #[serde(
        rename = "conan_redirect_alpha",
        alias = "78643fe5-d192-40c7-8e93-5ccf04c0b767"
    )]
    ConanRedirectAlpha,
    #[serde(
        rename = "conan_redirect_beta",
        alias = "bfeaee4e-965a-4d24-b163-020c3c57d936"
    )]
    ConanRedirectBeta,
    #[serde(
        rename = "conan_redirect_nightlies",
        alias = "98872b06-2ff3-4b71-96bc-039e2ebe7adc"
    )]
    ConanRedirectNightlies,
    #[serde(
        rename = "conan_redirect_nightly",
        alias = "7e7596aa-6e2c-41d1-a460-1e13cf0b62f2"
    )]
    ConanRedirectNightly,
    #[serde(
        rename = "conan_redirect_public",
        alias = "b20d91ca-1b2f-45a2-a115-c1ad24c66ac5"
    )]
    ConanRedirectPublic,
    #[serde(
        rename = "conan_redirect_qa",
        alias = "16d69c53-4c40-4821-b9f3-57ca690b2d4d"
    )]
    ConanRedirectQa,
    #[serde(
        rename = "content_filter",
        alias = "32cc8bf5-b425-4582-a52d-71b4f1cf436b"
    )]
    ContentFilter,
    #[serde(
        rename = "create_anonymous_users",
        alias = "f6a0e423-1a83-418c-8448-a1c7105fd71a"
    )]
    CreateAnonymousUsers,
    #[serde(
        rename = "CU Sunset",
        alias = "567033ef-ffee-44fb-8f90-f678077445f9"
    )]
    CuSunset,
    #[serde(
        rename = "custom-home-removal",
        alias = "849433b0-ef60-4a71-9dd9-939bc01f5362"
    )]
    CustomHomeRemoval,
    #[serde(rename = "download_certificates")]
    DownloadCertificates,
    #[serde(
        rename = "downloads-gating",
        alias = "3bfd3ccf-8c63-4dbb-8f87-9b21b402c82b"
    )]
    DownloadsGating,
    #[serde(
        rename = "drm_support",
        alias = "34e182bd-2f62-4678-a9e9-d13b3e25019d"
    )]
    DrmSupport,
    #[serde(
        rename = "dvr",
        alias = "e8230c74-0940-4b91-9e20-6571eb068086"
    )]
    Dvr,
    #[serde(
        rename = "dvr-block-unsupported-countries",
        alias = "c92d4903-bc06-4715-8ce4-4a22674abac8"
    )]
    DvrBlockUnsupportedCountries,
    #[serde(
        rename = "epg-recent-channels",
        alias = "9c982beb-c676-4d6f-a777-ff5d37ec3081"
    )]
    EpgRecentChannels,
    #[serde(
        rename = "exclude restrictions",
        alias = "6d7be725-9a96-42c7-8af4-01e735138822"
    )]
    ExcludeRestrictions,
    #[serde(
        rename = "federated-auth",
        alias = "5d819d02-5d04-4116-8eec-f49def4e2d6f"
    )]
    FederatedAuth,
    #[serde(
        rename = "global-continue-watching",
        alias = "4742780c-af9d-4b44-bf5b-7b27e3369aa8"
    )]
    GlobalContinueWatching,
    #[serde(
        rename = "grandfather-sync",
        alias = "7ee1495c-2798-4288-94e2-9cd98e67d441"
    )]
    GrandfatherSync,
    #[serde(
        rename = "guided-upgrade",
        alias = "c9d9b7ee-fdd9-474e-b143-5039c04e9b9b"
    )]
    GuidedUpgrade,
    #[serde(
        rename = "hardware_transcoding",
        alias = "bc8d1fca-deb0-4d0a-a6f4-12cfd681002d"
    )]
    HardwareTranscoding,
    #[serde(
        rename = "home",
        alias = "b2403ac6-4885-4971-8b96-59353fd87c72"
    )]
    Home,
    #[serde(
        rename = "hwtranscode",
        alias = "84a754b0-d1ca-4433-af2d-c949bf4b4936"
    )]
    Hwtranscode,
    #[serde(
        rename = "imagga-v2",
        alias = "55b9f6ed-5d26-4d2d-a436-68882a9901b5"
    )]
    ImaggaV2,
    #[serde(
        rename = "intro-markers",
        alias = "b83c8dc9-5a01-4b7a-a7c9-5870c8a6e21b"
    )]
    IntroMarkers,
    #[serde(
        rename = "ios14-privacy-banner",
        alias = "39dbdd84-8339-4736-96a1-0eb105cc2e08"
    )]
    Ios14PrivacyBanner,
    #[serde(
        rename = "item_clusters",
        alias = "b58d7f28-7b4a-49bb-97a7-152645505f28"
    )]
    ItemClusters,
    #[serde(
        rename = "kevin-bacon",
        alias = "c7ae6f8f-05e6-48bb-9024-c05c1dc3c43e"
    )]
    KevinBacon,
    #[serde(
        rename = "lets_encrypt",
        alias = "8536058d-e1dd-4ae7-b30f-e8b059b7cc17"
    )]
    LetsEncrypt,
    #[serde(
        rename = "lightning-dvr-pivot",
        alias = "6b85840c-d79d-40c2-8d8f-dfc0b7d26776"
    )]
    LightningDvrPivot,
    #[serde(
        rename = "live-tv-channels-grid",
        alias = "9eaa5152-320b-48e6-9d47-9492ba5e5b54"
    )]
    LiveTvChannelsGrid,
    #[serde(
        rename = "live-tv-grid-pagination",
        alias = "d1b1e233-a891-45e5-935c-6114e905dbe8"
    )]
    LiveTvGridPagination,
    #[serde(
        rename = "live-tv-on-plex-subtitles",
        alias = "0b8bf267-1acf-4f89-99eb-4afbb9d250e5"
    )]
    LiveTvOnPlexSubtitles,
    #[serde(
        rename = "live-tv-support-incomplete-segments",
        alias = "dab501df-5d99-48ef-afc2-3e839e4ddc9a"
    )]
    LiveTvSupportIncompleteSegments,
    #[serde(
        rename = "livetv",
        alias = "65685ff8-4375-4e4c-a806-ec1f0b4a8b7f"
    )]
    Livetv,
    #[serde(
        rename = "livetv-platform-specific",
        alias = "de789b83-9c5e-4472-bccf-791c69e67500"
    )]
    LivetvPlatformSpecific,
    #[serde(rename = "loudness")]
    Loudness,
    #[serde(
        rename = "lyrics",
        alias = "002c9f1a-2fc0-4812-b85b-0e6140f21a0f"
    )]
    Lyrics,
    #[serde(
        rename = "metadata",
        alias = "f8ea4f37-c554-476a-8852-1cbd2912f3f6"
    )]
    Metadata,
    #[serde(
        rename = "metadata_search",
        alias = "22b27e12-472e-4383-92ea-2ec3976d8e72"
    )]
    MetadataSearch,
    #[serde(
        rename = "music",
        alias = "e7cea823-02e5-48c4-a501-d37b82bf132f"
    )]
    Music,
    #[serde(
        rename = "music-analysis",
        alias = "8fd37970-6e4e-4f00-a64a-e70b52f18e94"
    )]
    MusicAnalysis,
    #[serde(
        rename = "music_preview",
        alias = "8e8dd5c8-14a4-4208-97d4-623e09191774"
    )]
    MusicPreview,
    #[serde(
        rename = "music_videos",
        alias = "1844737f-1a87-45c3-ab20-01435959e63c"
    )]
    MusicVideos,
    #[serde(
        rename = "news-provider-sunset-modal",
        alias = "b77e6744-c18d-415a-8e7c-7aac5d7a7750"
    )]
    NewsProviderSunsetModal,
    #[serde(
        rename = "nominatim",
        alias = "0de6151c-e0dd-47c8-a81e-1acb977c7f0f"
    )]
    Nominatim,
    #[serde(
        rename = "optimize-server-users-endpoint",
        alias = "ddd730e1-a0a0-429f-a7d3-7c5001d24497"
    )]
    #[deprecated]
    OptimizeServerUsersEndpoint,
    #[serde(
        rename = "parental-controls",
        alias = "73d0bba4-a6ba-4114-bac3-3039c12e08fb"
    )]
    ParentalControls,
    #[serde(
        rename = "pass",
        alias = "82999dd3-a2be-482e-9f44-357879b4f603"
    )]
    Pass,
    #[serde(rename = "photo_autotags")]
    PhotoAutotags,
    #[serde(
        rename = "photos-favorites",
        alias = "96cac76e-c5bc-4596-87eb-4fdfef9aaa11"
    )]
    PhotosFavorites,
    #[serde(
        rename = "photos-metadata-edition",
        alias = "2ea0e464-ea4f-4be2-97c1-ce6ed4b377dd"
    )]
    PhotosMetadataEdition,
    #[serde(
        rename = "photos-v5",
        alias = "0a348865-4f87-46dc-8bb2-f37637975724"
    )]
    PhotosV5,
    #[serde(
        rename = "photosV6-edit",
        alias = "850f3d1e-3f38-44c1-9c0c-e3c9127b8b5a"
    )]
    Photosv6Edit,
    #[serde(
        rename = "photosV6-tv-albums",
        alias = "3a2b0cb6-1519-4431-98e2-823c248c70eb"
    )]
    Photosv6TvAlbums,
    #[serde(
        rename = "podcasts",
        alias = "1841971c-6be5-40e6-a211-7e189d767a78"
    )]
    Podcasts,
    #[serde(
        rename = "premium-dashboard",
        alias = "222020fb-1504-492d-af33-a0b80a49558a"
    )]
    PremiumDashboard,
    #[serde(
        rename = "premium_music_metadata",
        alias = "d413fb56-de7b-40e4-acd0-f3dbb7c9e104"
    )]
    PremiumMusicMetadata,
    #[serde(
        rename = "radio",
        alias = "300231e0-69aa-4dce-97f4-52d8c00e3e8c"
    )]
    Radio,
    #[serde(
        rename = "retro-games",
        alias = "4e27cf82-9fb6-4ebe-8e10-c48bfe6fbbb6"
    )]
    #[deprecated]
    RetroGames,
    #[serde(
        rename = "retro-games-plex-tv",
        alias = "a4bc568b-477f-4f36-894b-49e19f34353f"
    )]
    #[deprecated]
    RetroGamesPlexTv,
    #[serde(
        rename = "save-to-library",
        alias = "644c4466-05fa-45e0-a478-c594cf81778f"
    )]
    SaveToLibrary,
    #[serde(rename = "server-manager")]
    ServerManager,
    #[serde(
        rename = "session_bandwidth_restrictions",
        alias = "05690239-443e-43fb-bc1a-95b5d916ca63"
    )]
    SessionBandwidthRestrictions,
    #[serde(
        rename = "session_kick",
        alias = "4ca03b04-54c1-4f9f-aea2-f813ae48f317"
    )]
    SessionKick,
    #[serde(rename = "shared-radio")]
    SharedRadio,
    #[serde(
        rename = "signin_notification",
        alias = "1b3a63e4-c2f4-4011-a181-2343d3a97ef7"
    )]
    SigninNotification,
    #[serde(
        rename = "signin_with_apple",
        alias = "b5874ecb-6610-47b2-8906-1b5a897acb02"
    )]
    SigninWithApple,
    #[serde(
        rename = "silence-removal",
        alias = "a6e0a154-4735-4cbb-a6ec-7a0a146c8216"
    )]
    SilenceRemoval,
    #[serde(
        rename = "singleitemsharing",
        alias = "62b1e357-5450-41d8-9b60-c7705f750849"
    )]
    Singleitemsharing,
    #[serde(
        rename = "sleep-timer",
        alias = "c5adf9dc-af13-4a85-a24b-98de6fa2f595"
    )]
    SleepTimer,
    #[serde(
        rename = "sonos-client-feature",
        alias = "8a9471c4-13bd-435a-b5b8-4ca6e423f355"
    )]
    SonosClientFeature,
    #[serde(
        rename = "spotlight-style-hub",
        alias = "579156cf-0664-45b4-8b7b-dda400ac3e26"
    )]
    SpotlightStyleHub,
    #[serde(
        rename = "spring_serve_ad_provider",
        alias = "fec722a0-a6d4-4fbd-96dc-4ffb02b072c5"
    )]
    SpringServeAdProvider,
    #[serde(
        rename = "Subtitles on Demand",
        alias = "bb50c92f-b412-44fe-8d8a-b1684f212a44"
    )]
    SubtitlesOnDemand,
    #[serde(
        rename = "sweet-fades",
        alias = "95149521-f64b-46ea-825c-9114e56afd2c"
    )]
    SweetFades,
    #[serde(
        rename = "sync",
        alias = "9dc1df45-fb45-4be1-9ab2-eb23eb57f082"
    )]
    Sync,
    #[serde(
        rename = "Sync v3",
        alias = "67c80530-eae3-4500-a9fa-9b6947d0f6d1"
    )]
    SyncV3,
    #[serde(
        rename = "trailers",
        alias = "6380e085-02fe-43b5-8bff-380fa4f2423c"
    )]
    Trailers,
    #[serde(
        rename = "transcoder_cache",
        alias = "a3d2d5c4-46a0-436e-a2d6-80d26f32b369"
    )]
    TranscoderCache,
    #[serde(
        rename = "TREBLE-show-features",
        alias = "85ebfb7b-77fb-4afd-bb1a-2fe2fefdddbe"
    )]
    TrebleShowFeatures,
    #[serde(
        rename = "tunefind-clients",
        alias = "07f804e6-28e6-4beb-b5c3-f2aefc88b938"
    )]
    TunefindClients,
    #[serde(
        rename = "tunefind-vod",
        alias = "50a1cfe9-dac1-4722-aee8-cc22e9758dd6"
    )]
    TunefindVod,
    #[serde(
        rename = "tuner-sharing",
        alias = "4b522f91-ae89-4f62-af9c-76f44d8ef61c"
    )]
    TunerSharing,
    #[serde(
        rename = "two-factor-authentication",
        alias = "06d14b9e-2af8-4c2b-a4a1-ea9d5c515824"
    )]
    TwoFactorAuthentication,
    #[serde(
        rename = "two-factor-authentication-clients",
        alias = "20824f5c-6dd9-4655-9970-e7701a73c02a"
    )]
    TwoFactorAuthenticationClients,
    #[serde(
        rename = "type-first",
        alias = "d14556be-ae6d-4407-89d0-b83953f4789a"
    )]
    TypeFirst,
    #[serde(
        rename = "ultrablur",
        alias = "cec2152f-321a-4c24-8c6d-c2b35a624389"
    )]
    Ultrablur,
    #[serde(
        rename = "universal-search",
        alias = "8b46de05-1f96-4278-87b3-010ba5b1e386"
    )]
    UniversalSearch,
    #[serde(
        rename = "unsupportedtuners",
        alias = "b46d16ae-cbd6-4226-8ee9-ab2b27e5dd42"
    )]
    Unsupportedtuners,
    #[serde(
        rename = "upgrade-3ds2",
        alias = "547514ab-3284-46e5-af77-bbaff247e3fc"
    )]
    Upgrade3ds2,
    #[serde(
        rename = "user-profile-whats-new",
        alias = "90138784-37a0-4a6b-87f6-1146d6c628e6"
    )]
    #[deprecated]
    UserProfileWhatsNew,
    #[serde(
        rename = "users-and-sharing",
        alias = "096ab4b8-04d2-41f4-9602-f1d5b9e8c7cc"
    )]
    UsersAndSharing,
    #[serde(
        rename = "visualizers",
        alias = "1dd846ed-7cde-4dc5-8ef6-53d3ce8c4e9d"
    )]
    Visualizers,
    #[serde(
        rename = "vod",
        alias = "cc9bea3b-11ab-4402-a222-4958bb129cab"
    )]
    Vod,
    #[serde(
        rename = "vod_channels",
        alias = "5050545e-95c6-4a25-b94e-1fe5a5a603d0"
    )]
    VodChannels,
    #[serde(
        rename = "vod_cloudflare",
        alias = "68747f3a-ce13-46ce-9274-1e0544c9f500"
    )]
    VodCloudflare,
    #[serde(
        rename = "vod_subtitles",
        alias = "ed374ad1-1d36-4396-8794-f710011e4fed"
    )]
    VodSubtitles,
    #[serde(
        rename = "volume-leveling",
        alias = "bbf73498-4912-4d80-9560-47c4fe212cec"
    )]
    VolumeLeveling,
    #[serde(
        rename = "watch-together-20200520",
        alias = "65faa2d0-f57e-4c63-a6b6-f1baa48951b1"
    )]
    WatchTogether20200520,
    #[serde(
        rename = "watch-together-invite",
        alias = "f83450e2-759a-4de4-8b31-e4a163896d43"
    )]
    WatchTogetherInvite,
    #[serde(
        rename = "watchlist",
        alias = "f0c452ce-11e7-465f-be04-5fb0bf4bec48"
    )]
    Watchlist,
    #[serde(
        rename = "web-desktop-gracenote-banner",
        alias = "13056a62-9bd2-47cf-aba9-bab00095fd08"
    )]
    WebDesktopGracenoteBanner,
    #[serde(
        rename = "web-desktop-live-tv-chromecast-remote-player",
        alias = "b737075d-a6c5-4e8a-8ee9-7dc72d984062"
    )]
    WebDesktopLiveTvChromecastRemotePlayer,
    #[serde(
        rename = "web-desktop-v4-dvr-setup",
        alias = "f8484f94-92a8-4ca4-9f43-e83ab3f586c7"
    )]
    WebDesktopV4DvrSetup,
    #[serde(
        rename = "web-desktop-v4-home",
        alias = "29bca3b8-e40b-4c69-b71c-f88047240f9b"
    )]
    WebDesktopV4Home,
    #[serde(
        rename = "web-desktop-v4-pre-plays",
        alias = "591895c1-8c60-4eab-8096-3594bb190257"
    )]
    WebDesktopV4PrePlays,
    #[serde(
        rename = "web-log-viewer",
        alias = "740a75d2-6dba-4317-ba68-ed3d619d4c7a"
    )]
    WebLogViewer,
    #[serde(
        rename = "web_server_dashboard",
        alias = "5e2a89ec-fb26-4234-b66e-14d37f35dff2"
    )]
    WebServerDashboard,
    #[serde(
        rename = "web-share-v2",
        alias = "73c73f05-7131-41cd-86d7-b91301684bfe"
    )]
    WebShareV2,
    #[serde(
        rename = "webhooks",
        alias = "6f82ca43-6117-4e55-ae0e-5ea3b3e99a96"
    )]
    Webhooks,
    #[serde(
        rename = "webshows",
        alias = "55e1398c-930f-41c1-bead-f5c2e471bb25"
    )]
    Webshows,
    UnknownUuid(String),
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

impl ::std::str::FromStr for Feature {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> ::std::result::Result<Feature, Self::Err> {
        let result = serde_plain::from_str(s);

        #[cfg(not(feature = "tests_deny_unknown_fields"))]
        let is_unknown_value = matches!(result, Ok(Feature::UnknownValue));
        #[cfg(feature = "tests_deny_unknown_fields")]
        let is_unknown_value = result.is_err();

        if is_unknown_value
            && s.len() == 36
            && s.as_bytes()[8] == b'-'
            && s.as_bytes()[13] == b'-'
            && s.as_bytes()[18] == b'-'
            && s.as_bytes()[23] == b'-'
        {
            return Ok(Feature::UnknownUuid(s.to_string()));
        }

        result
    }
}
derive_display_from_serialize!(Feature);
