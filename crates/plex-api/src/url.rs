pub const MYPLEX_DEFAULT_API_URL: &str = "https://plex.tv/";

pub const MYPLEX_SIGNIN_PATH: &str = "/api/v2/users/signin";
pub const MYPLEX_USER_INFO_PATH: &str = "/api/v2/user";
pub const MYPLEX_SIGNOUT_PATH: &str = "/api/v2/users/signout";
pub const MYPLEX_CLAIM_TOKEN_PATH: &str = "/api/claim/token.json";
pub const MYPLEX_PRIVACY_PATH: &str = "/api/v2/user/privacy";
pub const MYPLEX_WEBHOOKS_PATH: &str = "/api/v2/user/webhooks";
pub const MYPLEX_DEVICES: &str = "/devices.xml";
pub const MYPLEX_RESOURCES: &str = "/api/resources"; // TODO: migrate to /api/v2/resources.json
pub const MYPLEX_FEATURES: &str = "/api/v2/features";
pub const MYPLEX_COMPANIONS: &str = "/api/v2/companions";
pub const MYPLEX_PROVIDERS: &str = "/media/providers";

pub const MYPLEX_ANNOUNCEMENTS: &str = "/api/announcements";

pub const MYPLEX_SERVERS: &str = "/api/v2/servers";

pub const MYPLEX_INVITES: &str = "/api/invites";
pub const MYPLEX_INVITES_SHARED_SERVERS: &str = "/api/v2/shared_servers";
pub const MYPLEX_INVITES_INVITE: &str = "/api/v2/friends/invite";
pub const MYPLEX_INVITES_FRIENDS: &str = "/api/v2/friends";

pub const MYPLED_SONOS_DOMAIN: &str = "https://sonos.plex.tv/";
pub const MYPLEX_SONOS_RESOURCES: &str = "/resources";

pub const MYPLEX_PINS: &str = "/api/v2/pins";
pub const MYPLEX_PINS_LINK: &str = "/api/v2/pins/link.json";

pub const MYPLEX_USERS: &str = "/api/home/users";
pub const MYPLEX_USER_SWITCH: &str = "/api/v2/home/users/{uuid}/switch"; // ?includeSubscriptions=1&includeProviders=1&includeSettings=1&includeSharedSettings=1&pin=0373

pub const SERVER_MEDIA_PROVIDERS: &str = "/media/providers";
pub const SERVER_MYPLEX_ACCOUNT: &str = "/myplex/account";
pub const SERVER_MYPLEX_CLAIM: &str = "/myplex/claim";
pub const SERVER_PREFS: &str = "/:/prefs";
pub const SERVER_TRANSCODE_SESSIONS: &str = "/transcode/sessions";
pub const SERVER_TRANSCODE_DECISION: &str = "/video/:/transcode/universal/decision";
pub const SERVER_TRANSCODE_DOWNLOAD: &str = "/video/:/transcode/universal";
pub const SERVER_TRANSCODE_ART: &str = "/photo/:/transcode";
pub const SERVER_SYSTEM_PROXY: &str = "/system/proxy";

pub const CLIENT_RESOURCES: &str = "/resources";
