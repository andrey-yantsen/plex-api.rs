#[cfg(feature = "test_connect_anonymous")]
use crate::tests::get_server_anonymous;
#[cfg(feature = "test_connect_authenticated")]
use crate::tests::get_server_authenticated;
use crate::SettingsMediaContainer;
use serde_json::from_str;

#[test]
fn decode_prefs() {
    let s = r##"
{
    "size": 132,
    "Setting": [
        {"id": "FriendlyName","label": "Friendly name","summary": "This name will be used to identify this media server to other computers on your network. If you leave it blank, your computer's name will be used instead.","type": "text","default": "","value": "LED-Kremen-286","hidden": false,"advanced": false,"group": "general"},
        {"id": "sendCrashReports","label": "Send crash reports to Plex","summary": "This helps us improve your experience.","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "general"},
        {"id": "FSEventLibraryUpdatesEnabled","label": "Scan my library automatically","summary": "Your library will be updated automatically when changes to library folders are detected.","type": "bool","default": false,"value": true,"hidden": false,"advanced": false,"group": "library"},
        {"id": "FSEventLibraryPartialScanEnabled","label": "Run a partial scan when changes are detected","summary": "When changes to library folders are detected, only scan the folder that changed.","type": "bool","default": false,"value": true,"hidden": false,"advanced": true,"group": "library"},
        {"id": "ScheduledLibraryUpdatesEnabled","label": "Scan my library periodically","summary": "","type": "bool","default": false,"value": true,"hidden": false,"advanced": false,"group": "library"},
        {"id": "ScheduledLibraryUpdateInterval","label": "Library scan interval","summary": "","type": "int","default": 3600,"value": 21600,"hidden": false,"advanced": false,"group": "library","enumValues": "900:every 15 minutes|1800:every 30 minutes|3600:hourly|7200:every 2 hours|21600:every 6 hours|43200:every 12 hours|86400:daily"},
        {"id": "DisplayNotifications","label": "Display notifications","summary": "Display notifications when updating libraries.","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "library"},
        {"id": "autoEmptyTrash","label": "Empty trash automatically after every scan","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "library"},
        {"id": "allowMediaDeletion","label": "Allow media deletion","summary": "The owner of the server will be allowed to delete media files from disk.","type": "bool","default": true,"value": true,"hidden": false,"advanced": true,"group": "library"},
        {"id": "allowMediaDeletionLanOnly","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": "library"},
        {"id": "OnDeckWindow","label": "Weeks to consider for On Deck and Continue Watching","summary": "Media that has not been watched in this many weeks will not appear in On Deck or Continue Watching.","type": "int","default": 16,"value": 16,"hidden": false,"advanced": true,"group": "library"},
        {"id": "CinemaTrailersType","label": "Choose Cinema Trailers from","summary": "","type": "int","default": 1,"value": 1,"hidden": false,"advanced": false,"group": "extras","enumValues": "0:All movies|1:Only unwatched movies"},
        {"id": "CinemaTrailersFromLibrary","label": "Include Cinema Trailers from movies in my library","summary": "","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "extras"},
        {"id": "CinemaTrailersFromTheater","label": "Include Cinema Trailers from new and upcoming movies in theaters","summary": "This feature is Plex Pass only.","type": "bool","default": false,"value": true,"hidden": false,"advanced": false,"group": "extras"},
        {"id": "CinemaTrailersFromBluRay","label": "Include Cinema Trailers from new and upcoming movies on Blu-ray","summary": "This feature is Plex Pass only.","type": "bool","default": false,"value": true,"hidden": false,"advanced": false,"group": "extras"},
        {"id": "CinemaTrailersPrerollID","label": "Cinema Trailers pre-roll video","summary": "Enter the full path to the pre-roll video file. If multiple paths separated by commas are entered, videos will be played sequentially. If multiple paths separated by semi-colons are used, a single pre-roll video will be chosen randomly from the list.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "extras"},
        {"id": "iTunesSharingEnabled","label": "Enable iTunes plugin","summary": "A server restart is required for a change to take effect.","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "channels"},
        {"id": "iTunesLibraryXmlPath","label": "iTunes library XML path","summary": "","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "channels"},
        {"id": "iPhotoSharingEnabled","label": "Enable iPhoto channel","summary": "A server restart is required for a change to take effect.","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "channels"},
        {"id": "iPhotoLibraryXmlPath","label": "iPhoto library XML path","summary": "","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "channels"},
        {"id": "ApertureSharingEnabled","label": "Enable Aperture channel","summary": "A server restart is required for a change to take effect.","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "channels"},
        {"id": "ApertureLibraryXmlPath","label": "Aperture library XML path","summary": "","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "channels"},
        {"id": "disableCapabilityChecking","label": "Disable capability checking","summary": "Capability checking ensures that plug-ins that are incompatible with this version of the server or the current client application you are using are hidden. Disabling capability checking is useful during development, but will enable access to plug-ins that may perform unreliably with certain client applications.","type": "bool","default": false,"value": false,"hidden": false,"advanced": true,"group": "channels"},
        {"id": "MachineIdentifier","label": "A unique identifier for the machine","summary": "","type": "text","default": "","value": "916b3c54-6fc4-4d14-b702-980db3818e21","hidden": true,"advanced": false,"group": ""},
        {"id": "AllowHighOutputBitrates","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ConfigurationUrl","label": "Web Manager URL","summary": "","type": "text","default": "http://127.0.0.1:32400/web","value": "http://127.0.0.1:32400/web","hidden": true,"advanced": false,"group": "network"},
        {"id": "showDockIcon","label": "Show in dock","summary": "","type": "bool","default": false,"value": false,"hidden": false,"advanced": false,"group": "general"},
        {"id": "LoginItemCheckEnabled","label": "Ask to add a login item for externally mounted drives","summary": "","type": "bool","default": true,"value": false,"hidden": false,"advanced": true,"group": "general"},
        {"id": "AcceptedEULA","label": "Has the user accepted the EULA","summary": "","type": "bool","default": false,"value": true,"hidden": true,"advanced": false,"group": ""},
        {"id": "LanguageInCloud","label": "Use language preferences from plex.tv","summary": "","type": "bool","default": false,"value": true,"hidden": true,"advanced": false,"group": ""},
        {"id": "ArticleStrings","label": "Comma-separated list of strings considered articles when sorting titles. A server restart is required for a change to take effect.","summary": "","type": "text","default": "the,das,der,a,an,el,la","value": "the,das,der,a,an,el,la","hidden": true,"advanced": false,"group": ""},
        {"id": "TranscoderCanOnlyRemuxVideo","label": "The transcoder can only remux video","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "TranscoderVideoResolutionLimit","label": "Maximum video output resolution for the transcoder","summary": "","type": "text","default": "0x0","value": "0x0","hidden": true,"advanced": false,"group": ""},
        {"id": "TranscoderPhotoFileSizeLimitMiB","label": "","summary": "","type": "int","default": 100,"value": 100,"hidden": true,"advanced": false,"group": ""},
        {"id": "EnableIPv6","label": "Enable server support for IPv6","summary": "","type": "bool","default": false,"value": false,"hidden": false,"advanced": true,"group": "network"},
        {"id": "secureConnections","label": "Secure connections","summary": "When set to \"Required\", some unencrypted connections (originating from the Media Server computer) will still be allowed and apps that don't support secure connections will not be able to connect at all.","type": "int","default": 1,"value": 1,"hidden": false,"advanced": false,"group": "network","enumValues": "0:Required|1:Preferred|2:Disabled"},
        {"id": "customCertificatePath","label": "Custom certificate location","summary": "Path to a PKCS #12 file containing a certificate and private key to enable TLS support on a custom domain.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network"},
        {"id": "customCertificateKey","label": "Custom certificate encryption key","summary": "","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network"},
        {"id": "customCertificateDomain","label": "Custom certificate domain","summary": "Domain name to be published to plex.tv using your mapped port; must match a name from the custom certificate file.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network"},
        {"id": "PreferredNetworkInterface","label": "Preferred network interface","summary": "The network interface local clients will use to connect.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network","enumValues": ":Any|en0:en0 (192.168.88.18)"},
        {"id": "GdmEnabled","label": "Enable local network discovery (GDM)","summary": "This enables the media server to discover other servers and players on the local network.","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "network"},
        {"id": "PublishServerOnPlexOnlineKey","label": "Publish server on Plex Online","summary": "Publishing a server makes it automatically available on your client devices without any configuration of your router.","type": "bool","default": false,"value": true,"hidden": true,"advanced": false,"group": ""},
        {"id": "PlexOnlineMail","label": "","summary": "","type": "text","default": "","value": "andrey@janzen.su","hidden": true,"advanced": false,"group": ""},
        {"id": "PlexOnlineUrl","label": "","summary": "","type": "text","default": "https://plex.tv","value": "https://plex.tv","hidden": true,"advanced": false,"group": ""},
        {"id": "ManualPortMappingMode","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ManualPortMappingPort","label": "","summary": "","type": "int","default": 32400,"value": 32400,"hidden": true,"advanced": false,"group": ""},
        {"id": "LastAutomaticMappedPort","label": "","summary": "","type": "int","default": 0,"value": 24507,"hidden": true,"advanced": false,"group": ""},
        {"id": "TranscoderQuality","label": "Transcoder quality","summary": "Quality profile used by the transcoder.","type": "int","default": 0,"value": 2,"hidden": false,"advanced": false,"group": "transcoder","enumValues": "0:Automatic|1:Prefer higher speed encoding|2:Prefer higher quality encoding|3:Make my CPU hurt"},
        {"id": "SegmentedTranscoderTimeout","label": "Segmented transcoder timeout","summary": "Timeout in seconds segmented transcodes wait for the transcoder to begin writing data.","type": "int","default": 20,"value": 20,"hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderTempDirectory","label": "Transcoder temporary directory","summary": "Directory to use when transcoding for temporary files.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "transcoder"},
        {"id": "TranscoderDefaultDuration","label": "Transcoder default duration","summary": "Duration in minutes to use when transcoding something with an unknown duration.","type": "int","default": 120,"value": 120,"hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderThrottleBuffer","label": "Transcoder default throttle buffer","summary": "Amount in seconds to buffer before throttling the transcoder.","type": "int","default": 60,"value": 60,"hidden": false,"advanced": true,"group": "transcoder"},
        {"id": "TranscoderPruneBuffer","label": "Transcoder default prune buffer","summary": "Amount in past seconds to retain before pruning segments from a transcode.","type": "int","default": 300,"value": 300,"hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderLivePruneBuffer","label": "","summary": "","type": "int","default": 5400,"value": 5400,"hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderH264Preset","label": "","summary": "","type": "text","default": "veryfast","value": "veryfast","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderH264BackgroundPreset","label": "Background transcoding x264 preset","summary": "The x264 preset value used for background transcoding (Sync and Media Optimizer). Slower values will result in better video quality and smaller file sizes, but will take significantly longer to complete processing.","type": "text","default": "veryfast","value": "slower","hidden": false,"advanced": true,"group": "transcoder","enumValues": "ultrafast:Ultra fast|superfast:Super fast|veryfast:Very fast|faster:Faster|fast:Fast|medium:Medium|slow:Slow|slower:Slower|veryslow:Very slow"},
        {"id": "TranscoderH264Options","label": "","summary": "","type": "text","default": "","value": "","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderH264OptionsOverride","label": "","summary": "","type": "text","default": "","value": "","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderH264MinimumCRF","label": "","summary": "","type": "double","default": "16","value": "16","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "TranscoderLogLevel","label": "","summary": "","type": "text","default": "error","value": "error","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "HardwareAcceleratedCodecs","label": "Use hardware acceleration when available","summary": "Plex Media Server will attempt to use hardware-accelerated video codecs when encoding and decoding video. Hardware acceleration can make transcoding faster and allow more simultaneous video transcodes, but it can also reduce video quality and compatibility.","type": "bool","default": false,"value": true,"hidden": false,"advanced": true,"group": "transcoder"},
        {"id": "SystemAudioCodecs","label": "","summary": "","type": "bool","default": true,"value": true,"hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "HardwareDevicePath","label": "","summary": "","type": "text","default": "/dev/dri/renderD128","value": "/dev/dri/renderD128","hidden": true,"advanced": false,"group": "transcoder"},
        {"id": "PluginsLaunchTimeout","label": "Number of seconds to wait before a plugin times out","summary": "","type": "int","default": 180,"value": 180,"hidden": true,"advanced": false,"group": "channels"},
        {"id": "DlnaEnabled","label": "Enable the DLNA server","summary": "This allows the server to stream media to DLNA (Digital Living Network Alliance) devices.","type": "bool","default": true,"value": false,"hidden": false,"advanced": false,"group": "dlna"},
        {"id": "DlnaPlatinumLoggingLevel","label": "DLNA server logging level","summary": "","type": "text","default": "OFF","value": "OFF","hidden": true,"advanced": false,"group": "dlna","enumValues": "OFF|FATAL|SEVERE|WARNING|INFO|FINE|FINER|FINEST|ALL"},
        {"id": "DlnaClientPreferences","label": "DLNA client preferences","summary": "Client-specific configuration settings for the DLNA server.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "dlna"},
        {"id": "DlnaReportTimeline","label": "DLNA server timeline reporting","summary": "Enable the DLNA server to report timelines for video play activity.","type": "bool","default": true,"value": true,"hidden": false,"advanced": true,"group": "dlna"},
        {"id": "DlnaDefaultProtocolInfo","label": "DLNA default protocol info","summary": "Protocol info string used in GetProtocolInfo responses by the DLNA server.","type": "text","default": "http-get:*:video/mpeg:*,http-get:*:video/mp4:*,http-get:*:video/vnd.dlna.mpeg-tts:*,http-get:*:video/avi:*,http-get:*:video/x-matroska:*,http-get:*:video/x-ms-wmv:*,http-get:*:video/wtv:*,http-get:*:audio/mpeg:*,http-get:*:audio/mp3:*,http-get:*:audio/mp4:*,http-get:*:audio/x-ms-wma*,http-get:*:audio/wav:*,http-get:*:audio/L16:*,http-get:*image/jpeg:*,http-get:*image/png:*,http-get:*image/gif:*,http-get:*image/tiff:*","value": "http-get:*:video/mpeg:*,http-get:*:video/mp4:*,http-get:*:video/vnd.dlna.mpeg-tts:*,http-get:*:video/avi:*,http-get:*:video/x-matroska:*,http-get:*:video/x-ms-wmv:*,http-get:*:video/wtv:*,http-get:*:audio/mpeg:*,http-get:*:audio/mp3:*,http-get:*:audio/mp4:*,http-get:*:audio/x-ms-wma*,http-get:*:audio/wav:*,http-get:*:audio/L16:*,http-get:*image/jpeg:*,http-get:*image/png:*,http-get:*image/gif:*,http-get:*image/tiff:*","hidden": false,"advanced": true,"group": "dlna"},
        {"id": "DlnaDeviceDiscoveryInterval","label": "DLNA media renderer discovery interval","summary": "Number of seconds between DLNA media renderer discovery requests.","type": "int","default": 60,"value": 60,"hidden": false,"advanced": true,"group": "dlna"},
        {"id": "DlnaAnnouncementLeaseTime","label": "DLNA server announcement lease time","summary": "Duration in seconds of DLNA Server SSDP announcement lease time.","type": "int","default": 1800,"value": 1800,"hidden": false,"advanced": true,"group": "dlna"},
        {"id": "DlnaDescriptionIcons","label": "DLNA server description icons","summary": "Icons offered by DLNA server when devices request server description.","type": "text","default": "png,jpeg;260x260,120x120,48x48","value": "png,jpeg;260x260,120x120,48x48","hidden": false,"advanced": true,"group": "dlna"},
        {"id": "SyncMyPlexLoginGCDeferral","label": "","summary": "","type": "int","default": 14400,"value": 14400,"hidden": true,"advanced": false,"group": ""},
        {"id": "SyncPagingItemsLimit","label": "","summary": "","type": "int","default": 100,"value": 100,"hidden": true,"advanced": false,"group": ""},
        {"id": "BackgroundQueueIdlePaused","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ScannerLowPriority","label": "Run scanner tasks at a lower priority","summary": "","type": "bool","default": false,"value": true,"hidden": false,"advanced": true,"group": "library"},
        {"id": "WanPerStreamMaxUploadRate","label": "Limit remote stream bitrate","summary": "Set the maximum bitrate of a remote stream from this server.","type": "int","default": 0,"value": 12000,"hidden": true,"advanced": false,"group": "","enumValues": "0:Original (No limit)|20000:20 Mbps (1080p)|12000:12 Mbps (1080p)|10000:10 Mbps (1080p)|8000:8 Mbps (1080p)|4000:4 Mbps (720p)|3000:3 Mbps (720p)|2000:2 Mbps (480p)|1500:1.5 Mbps (480p)|720:720 kbps|320:320 kbps"},
        {"id": "WanTotalMaxUploadRate","label": "External network total upload limit (kbps)","summary": "Speed at which to limit the total bandwidth not on the local network in kilobits per second. Use 0 to set no limit.","type": "int","default": 0,"value": 50000,"hidden": true,"advanced": false,"group": ""},
        {"id": "WanPerUserStreamCount","label": "Remote streams allowed per user","summary": "Maximum number of simultaneous streams each user is allowed when not on the local network.","type": "int","default": 0,"value": 0,"hidden": false,"advanced": true,"group": "network","enumValues": "0:Unlimited|1:1|2:2|3:3|4:4|5:5|6:6|7:7|8:8|9:9|10:10|11:11|12:12|13:13|14:14|15:15|16:16|17:17|18:18|19:19|20:20"},
        {"id": "LanNetworksBandwidth","label": "LAN Networks","summary": "Comma separated list of IP addresses or IP/netmask entries for networks that will be considered to be on the local network when enforcing bandwidth restrictions. If set, all other IP addresses will be considered to be on the external network and and will be subject to external network bandwidth restrictions. If left blank, only the server's subnet is considered to be on the local network.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network"},
        {"id": "forceAutoAdjustQuality","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "EnableABRDebugOverlay","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ABRKeepOldTranscodes","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ForceABRDisabled","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "TreatWanIpAsLocal","label": "Treat WAN IP As LAN Bandwidth","summary": "Treat incoming requests from this network's WAN IP address as LAN requests in terms of bandwidth.  This often occurs when DNS rebinding protection is in place and clients on the LAN cannot contact the server directly but instead have to go through the WAN IP address.","type": "bool","default": true,"value": true,"hidden": false,"advanced": true,"group": "network"},
        {"id": "TranscodeCountLimit","label": "Maximum simultaneous video transcode","summary": "Limit the number of simultaneous video transcode streams your server can utilize","type": "int","default": 0,"value": 3,"hidden": false,"advanced": false,"group": "transcoder","enumValues": "0:Unlimited|1:1|2:2|3:3|4:4|5:5|6:6|7:7|8:8|9:9|10:10|11:11|12:12|13:13|14:14|15:15|16:16|17:17|18:18|19:19|20:20"},
        {"id": "logDebug","label": "Enable Plex Media Server debug logging","summary": "","type": "bool","default": true,"value": false,"hidden": false,"advanced": true,"group": "general"},
        {"id": "LogVerbose","label": "Enable Plex Media Server verbose logging","summary": "","type": "bool","default": false,"value": false,"hidden": false,"advanced": true,"group": "general"},
        {"id": "logTokens","label": "Allow Plex Media Server tokens in logs","summary": "Media server tokens can be used to gain access to library content. Don't share logs containing tokens publicly. A server restart is required for a change to take effect.","type": "bool","default": false,"value": false,"hidden": false,"advanced": true,"group": "general"},
        {"id": "MinimumProgressTime","label": "","summary": "","type": "int","default": 60000,"value": 60000,"hidden": true,"advanced": false,"group": "general"},
        {"id": "customConnections","label": "Custom server access URLs","summary": "A comma-separated list of URLs (http or https) which are published up to plex.tv for server discovery.","type": "text","default": "","value": "","hidden": false,"advanced": true,"group": "network"},
        {"id": "allowedNetworks","label": "List of IP addresses and networks that are allowed without auth","summary": "Comma separated list of IP addresses or IP/netmask entries for networks that are allowed to access Plex Media Server without logging in. When the server is signed out and this value is set, only localhost and addresses on this list will be allowed.","type": "text","default": "","value": "192.168.88.0/24","hidden": false,"advanced": true,"group": "network"},
        {"id": "enableAirplay","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": "network"},
        {"id": "enableHttpPipelining","label": "Enable HTTP Pipelining","summary": "This feature can enable higher performance in the HTTP server component. A server restart is required for a change to take effect.","type": "bool","default": true,"value": true,"hidden": false,"advanced": true,"group": "network"},
        {"id": "WebHooksEnabled","label": "Webhooks","summary": "This feature enables your server to send events to external services.","type": "bool","default": true,"value": true,"hidden": false,"advanced": true,"group": "network"},
        {"id": "ButlerStartHour","label": "Time at which tasks start to run","summary": "The time at which the server starts running background maintenance tasks.","type": "int","default": 2,"value": 1,"hidden": false,"advanced": false,"group": "butler","enumValues": "0:Midnight|1:1 am|2:2 am|3:3 am|4:4 am|5:5 am|6:6 am|7:7 am|8:8 am|9:9 am|10:10 am|11:11 am|12:Noon|13:1 pm|14:2 pm|15:3 pm|16:4 pm|17:5 pm|18:6 pm|19:7 pm|20:8 pm|21:9 pm|22:10 pm|23:11 pm"},
        {"id": "ButlerEndHour","label": "Time at which tasks stop running","summary": "The time at which the background maintenance tasks stop running.","type": "int","default": 5,"value": 7,"hidden": false,"advanced": false,"group": "butler","enumValues": "0:Midnight|1:1 am|2:2 am|3:3 am|4:4 am|5:5 am|6:6 am|7:7 am|8:8 am|9:9 am|10:10 am|11:11 am|12:Noon|13:1 pm|14:2 pm|15:3 pm|16:4 pm|17:5 pm|18:6 pm|19:7 pm|20:8 pm|21:9 pm|22:10 pm|23:11 pm"},
        {"id": "ButlerTaskBackupDatabase","label": "Backup database every three days","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerDatabaseBackupPath","label": "Backup directory","summary": "The directory in which database backups are stored.","type": "text","default": "/Users/virus/Library/Application Support/Plex Media Server/Plug-in Support/Databases","value": "/Users/virus/Library/Application Support/Plex Media Server/Plug-in Support/Databases","hidden": false,"advanced": true,"group": "butler"},
        {"id": "ButlerTaskOptimizeDatabase","label": "Optimize database every week","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskCleanOldBundles","label": "Remove old bundles every week","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskCleanOldCacheFiles","label": "Remove old cache files every week","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskRefreshLocalMedia","label": "Refresh local metadata every three days","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskRefreshLibraries","label": "Update all libraries during maintenance","summary": "","type": "bool","default": false,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskUpgradeMediaAnalysis","label": "Upgrade media analysis during maintenance","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskRefreshPeriodicMetadata","label": "Refresh metadata periodically","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskDeepMediaAnalysis","label": "Perform extensive media analysis during maintenance","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskRefreshEpgGuides","label": "Perform refresh of program guide data.","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskReverseGeocode","label": "Fetch missing location names for items in photo sections","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskGenerateAutoTags","label": "Analyze and tag photos","summary": "","type": "bool","default": true,"value": true,"hidden": false,"advanced": false,"group": "butler"},
        {"id": "ButlerTaskUpdateServer","label": "Server version updates","summary": "","type": "text","default": "askme","value": "always","hidden": false,"advanced": false,"group": "general","enumValues": "askme:Ask me|always:Automatically during scheduled maintenance"},
        {"id": "ButlerTaskUpdateScheduled","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": "general"},
        {"id": "ButlerTaskUpdateVersionSkipped","label": "","summary": "","type": "text","default": "","value": "1.12.1.4885-1046ba85f","hidden": true,"advanced": false,"group": "general"},
        {"id": "ButlerUpdateChannel","label": "Server update Channel","summary": "","type": "text","default": "16","value": "0","hidden": false,"advanced": false,"group": "general","enumValues": "0:Public|8:Beta"},
        {"id": "GenerateIndexFilesDuringAnalysis","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "ButlerTaskGenerateMediaIndexFiles","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "GenerateBIFBehavior","label": "Generate video preview thumbnails","summary": "Video preview thumbnails provide live updates in Now Playing and while seeking on supported apps. Thumbnail generation may take a long time, cause high CPU usage, and consume additional disk space. You can turn off thumbnail generation for individual libraries in the library's advanced settings.","type": "text","default": "never","value": "scheduled","hidden": false,"advanced": false,"group": "library","enumValues": "never:never|scheduled:as a scheduled task|asap:as a scheduled task and when media is added"},
        {"id": "GenerateChapterThumbBehavior","label": "Generate chapter thumbnails","summary": "Chapter thumbnails provide images in the chapter view on supported apps. They can take a long time to generate and consume additional disk space.","type": "text","default": "scheduled","value": "scheduled","hidden": false,"advanced": false,"group": "library","enumValues": "never:never|scheduled:as a scheduled task|asap:as a scheduled task and when media is added"},
        {"id": "LoudnessAnalysisBehavior","label": "Analyze audio tracks for loudness","summary": "Loudness analysis allows various features, such as loudness leveling and smart transitions. It can take a long time to complete when analyzing many tracks, and cause high CPU usage.","type": "text","default": "scheduled","value": "scheduled","hidden": false,"advanced": false,"group": "library","enumValues": "never:never|scheduled:as a scheduled task|asap:as a scheduled task and when media is added"},
        {"id": "LoudnessAnalysisThreads","label": "","summary": "","type": "int","default": 0,"value": 0,"hidden": true,"advanced": false,"group": ""},
        {"id": "RadioTopTracksPerAlbum","label": "","summary": "","type": "int","default": 3,"value": 3,"hidden": true,"advanced": false,"group": ""},
        {"id": "RadioDaysSinceLastPlayed","label": "","summary": "","type": "int","default": 2,"value": 2,"hidden": true,"advanced": false,"group": ""},
        {"id": "LocationVisibility","label": "Location visibility","summary": "Server owners may wish to restrict who can see location names for items which contain geolocation metadata. By default only the server owner will have visibility of these.","type": "int","default": 1,"value": 1,"hidden": false,"advanced": false,"group": "library","enumValues": "1:admin only|2:everyone"},
        {"id": "GracenoteUser","label": "","summary": "","type": "text","default": "","value": "WEcxA+k0zYP6loQ/lsxo/NLNA/sgJankXuyy7Rq/0kpM7TmZBuOgcpGAIElr+mDMiq4SLuoL5gX+F2Dgob6WUXO4qnV6oLd6pJ/TBHXsZpCcVZv0KItIKQ3d3q1vrvNaGxTqRJ2XHVjCX2u7aUaLVHXHyt0QTZt1v+Hv4L6KShDUg1raM90a61uiQS3pceANSHhu","hidden": true,"advanced": false,"group": ""},
        {"id": "CertificateVersion","label": "","summary": "","type": "int","default": 2,"value": 2,"hidden": true,"advanced": false,"group": ""},
        {"id": "EyeQUser","label": "","summary": "","type": "text","default": "","value": "","hidden": true,"advanced": false,"group": ""},
        {"id": "DvrShowUnsupportedDevices","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "DvrComskipRemoveIntermediates","label": "","summary": "","type": "bool","default": true,"value": true,"hidden": true,"advanced": false,"group": ""},
        {"id": "DvrComskipKeepOriginal","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "DvrOnConnectTestingUrl","label": "","summary": "","type": "text","default": "","value": "","hidden": true,"advanced": false,"group": ""},
        {"id": "SubtitlesPersistIfAdmin","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""},
        {"id": "DvrIncrementalEpgLoader","label": "","summary": "","type": "bool","default": false,"value": false,"hidden": true,"advanced": false,"group": ""}
    ]
}
    "##;

    let mc = from_str::<SettingsMediaContainer>(s);
    assert!(
        dbg!(&mc).is_ok(),
        "Unable to deserialize prefs: {:?}",
        mc.err()
    );
}

#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_settings_online_authenticated() {
    let srv = get_server_authenticated().await;
    let settings = srv.get_settings().await;
    assert!(
        settings.is_ok(),
        "Unable to get settings: {:?}",
        settings.err()
    );
}

#[cfg(feature = "test_connect_anonymous")]
#[tokio::test]
async fn decode_settings_online_anonymous() {
    let srv = get_server_anonymous().await;
    let settings = srv.get_settings().await;
    assert!(
        settings.is_ok(),
        "Unable to get settings: {:?}",
        settings.err()
    );
}
