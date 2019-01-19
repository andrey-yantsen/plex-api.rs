use crate::{bool_from_int, option_bool_from_int};
use chrono::{DateTime, Utc};
use serde_with::CommaSeparator;

#[derive(Debug, Deserialize)]
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
}

impl MediaContainer {
    pub fn get_devices(&self) -> Option<Vec<Device>> {
        self.devices.clone()
    }
    pub fn get_users(&self) -> Option<Vec<User>> {
        self.users.clone()
    }
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

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Device {
    name: String,
    #[serde(rename = "publicAddress")]
    public_address: String,
    product: String,
    #[serde(rename = "productVersion")]
    product_version: String,
    platform: String,
    #[serde(rename = "platformVersion")]
    platform_version: String,
    device: String,
    model: Option<String>,
    vendor: Option<String>,
    #[serde(
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize"
    )]
    provides: Vec<String>,
    #[serde(rename = "clientIdentifier")]
    client_identifier: String,
    version: Option<String>,
    id: Option<u32>,
    token: Option<String>,
    #[serde(rename = "accessToken")]
    access_token: Option<String>,
    #[serde(rename = "createdAt", with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(rename = "lastSeenAt", with = "chrono::serde::ts_seconds")]
    last_seen_at: DateTime<Utc>,
    #[serde(
        rename = "screenResolution",
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize",
        default
    )]
    screen_resolution: Vec<String>,
    #[serde(
        rename = "screenDensity",
        deserialize_with = "serde_with::rust::string_empty_as_none::deserialize",
        default
    )]
    screen_density: Option<u8>,
    #[serde(rename = "Connection")]
    connections: Option<Vec<Connection>>,
    #[serde(
        rename = "httpsRequired",
        deserialize_with = "option_bool_from_int",
        default
    )]
    https_required: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    synced: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
    #[serde(
        rename = "publicAddressMatches",
        deserialize_with = "option_bool_from_int",
        default
    )]
    public_address_matches: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    presence: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    owned: Option<bool>,
    #[serde(rename = "SyncList")]
    sync_list: Option<SyncList>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct SyncList {
    #[serde(rename = "itemsCompleteCount")]
    items_complete_count: u32,
    #[serde(rename = "totalSize")]
    total_size: u64,
    version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Connection {
    uri: String,
    protocol: Option<String>,
    address: Option<String>,
    port: Option<u32>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    local: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
}
