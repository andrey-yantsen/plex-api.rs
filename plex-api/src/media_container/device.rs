use std::sync::mpsc;
use std::thread;

use chrono::{DateTime, Utc};
use serde_with::CommaSeparator;

use async_std::task::block_on;

use crate::serde_helpers::option_bool_from_int;
use crate::server::Server;
use crate::{HasBaseUrl, HasDeleteUrl, HasMyPlexToken, MediaContainer};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct DevicesMediaContainer {
    #[serde(rename = "Device")]
    devices: Option<Vec<Device>>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

impl DevicesMediaContainer {
    pub fn get_media_container(self) -> MediaContainer {
        self.media_container
    }
    pub fn get_devices(self) -> Option<Vec<Device>> {
        self.devices
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
    #[serde(deserialize_with = "option_bool_from_int", default)]
    https_required: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    synced: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    public_address_matches: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    presence: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
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
    uri: String,
    protocol: Option<String>,
    address: Option<String>,
    port: Option<u32>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    local: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
}

impl Device {
    pub fn connect_to_server(&self) -> crate::Result<Server> {
        // TODO: Make async
        if !self.provides.contains(&String::from("server")) {
            // TODO: Return descriptive error
            return Err(crate::error::PlexApiError {});
        }

        if self.connections.is_none() {
            // TODO: Return descriptive error
            return Err(crate::error::PlexApiError {});
        }

        let (tx, rx) = mpsc::channel();
        let mut handlers = vec![];
        let connections = self.connections.clone().unwrap();
        for c in connections {
            let tx_clone = mpsc::Sender::clone(&tx);
            let auth_token_clone = self.auth_token.clone();
            let handler = thread::spawn(move || {
                let srv = if auth_token_clone.is_empty() {
                    block_on(Server::connect(&c.uri))
                } else {
                    block_on(Server::login(&c.uri, &auth_token_clone))
                };
                tx_clone.send(srv)
            });
            handlers.push(handler);
        }

        let mut left = handlers.len();

        loop {
            let thread_result = rx.recv();
            if let Ok(srv) = thread_result {
                if srv.is_ok() {
                    return srv;
                }
            }
            left -= 1;
            if left == 0 {
                // TODO: Return descriptive error
                return Err(crate::error::PlexApiError {});
            }
        }
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
