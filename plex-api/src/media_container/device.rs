use chrono::{DateTime, Utc};
use serde_with::CommaSeparator;

use crate::serde_helpers::option_bool_from_anything;
use crate::server::Server;
use crate::{HasBaseUrl, HasDeleteUrl, HasMyPlexToken, MediaContainer, PlexApiError};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DevicesMediaContainer {
    #[serde(rename = "Device", default)]
    devices: Vec<Device>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

impl DevicesMediaContainer {
    pub const fn get_media_container(&self) -> &MediaContainer {
        &self.media_container
    }
    pub const fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Device {
    name: String,
    public_address: String,
    product: String,
    product_version: String,
    platform: String,
    platform_version: String,
    device: String,
    model: Option<String>,
    vendor: Option<String>,
    #[serde(
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize"
    )]
    provides: Vec<String>,
    client_identifier: String,
    version: Option<String>,
    id: Option<u32>,
    token: Option<String>,
    access_token: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    last_seen_at: DateTime<Utc>,
    #[serde(
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize",
        default
    )]
    screen_resolution: Vec<String>,
    #[serde(
        deserialize_with = "serde_with::rust::string_empty_as_none::deserialize",
        default
    )]
    screen_density: Option<u16>,
    #[serde(rename = "Connection")]
    connections: Option<Vec<Connection>>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    https_required: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    synced: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    relay: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    public_address_matches: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    presence: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    owned: Option<bool>,
    #[serde(rename = "SyncList")]
    sync_list: Option<SyncList>,
    #[serde(default)]
    auth_token: String,
    dns_rebinding_protection: Option<bool>,
    nat_loopback_supported: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct SyncList {
    items_complete_count: u32,
    total_size: u64,
    version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct Connection {
    uri: Url,
    protocol: Option<String>,
    address: Option<String>,
    port: Option<u32>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    local: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_anything", default)]
    relay: Option<bool>,
}

impl Device {
    pub async fn connect_to_server(&self) -> crate::Result<Server> {
        // TODO: Try servers in parallel
        if !self.provides.contains(&String::from("server")) {
            return Err(PlexApiError::CurrentDeviceIsNotServer {
                provides: self.provides.clone(),
            });
        }

        if self.connections.is_none() {
            return Err(PlexApiError::EmptyConnectionsList);
        }

        let mut connections = self.connections.clone().unwrap();

        connections.sort_by(|a, b| a.local.cmp(&b.local));

        let mut errors = HashMap::new();

        for c in connections {
            let srv = Server::connect(c.uri.clone(), &self.auth_token).await;

            if srv.is_ok() {
                return srv;
            } else {
                errors.insert(c.uri.clone(), srv.err().unwrap());
            }
        }

        Err(PlexApiError::ConnectionFailed { errors })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasMyPlexToken for Device {
    /// Returns authentication token for current account.
    fn get_auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Sets authentication token for current account.
    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = String::from(auth_token);
    }
}

impl HasDeleteUrl for Device {
    fn get_delete_url(&self) -> Option<String> {
        self.id
            .map(|id| format!("https://plex.tv/devices/{}.xml", id))
    }
}

// Required to have default implementation of CanMakeRequests for Device
impl HasBaseUrl for Device {
    fn get_base_url(&self) -> &str {
        unimplemented!()
    }
}
