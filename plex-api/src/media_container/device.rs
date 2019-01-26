use crate::serde_helpers::option_bool_from_int;
use crate::server::{Server, ServerError};
use chrono::{DateTime, Utc};
use serde_with::CommaSeparator;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "strict_deserialize", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "strict_deserialize", serde(deny_unknown_fields))]
struct SyncList {
    #[serde(rename = "itemsCompleteCount")]
    items_complete_count: u32,
    #[serde(rename = "totalSize")]
    total_size: u64,
    version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "strict_deserialize", serde(deny_unknown_fields))]
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
    pub fn connect_to_server(&self) -> Result<Server, ServerError> {
        if !self.provides.contains(&String::from("server")) {
            // TODO: Return descriptive error
            return Err(ServerError {});
        }

        if self.connections.is_none() {
            // TODO: Return descriptive error
            return Err(ServerError {});
        }

        let (tx, rx) = mpsc::channel();
        let mut handlers = vec![];
        let connections = self.connections.clone().unwrap();
        for c in connections {
            let tx_clone = mpsc::Sender::clone(&tx);
            let handler = thread::spawn(move || {
                let srv = Server::connect(&c.uri);
                tx_clone.send(srv)
            });
            handlers.push(handler);
        }

        let mut left = handlers.len();

        loop {
            let thread_result = rx.recv();
            if thread_result.is_ok() {
                let srv = thread_result.unwrap();
                if srv.is_ok() {
                    return srv;
                }
            }
            left -= 1;
            if left == 0 {
                // TODO: Return descriptive error
                return Err(ServerError {});
            }
        }
    }

    pub fn login_to_server(&self, auth_token: &str) -> Result<Server, ServerError> {
        if !self.provides.contains(&String::from("server")) {
            // TODO: Return descriptive error
            return Err(ServerError {});
        }

        if self.connections.is_none() {
            // TODO: Return descriptive error
            return Err(ServerError {});
        }

        let (tx, rx) = mpsc::channel();
        let mut handlers = vec![];
        let connections = self.connections.clone().unwrap();
        for c in connections {
            let tx_clone = mpsc::Sender::clone(&tx);
            let auth_token_clone = String::from(auth_token);
            let handler = thread::spawn(move || {
                let srv = Server::login(&c.uri, &auth_token_clone);
                tx_clone.send(srv)
            });
            handlers.push(handler);
        }

        let mut left = handlers.len();

        loop {
            let thread_result = rx.recv();
            if thread_result.is_ok() {
                let srv = thread_result.unwrap();
                if srv.is_ok() {
                    return srv;
                }
            }
            left -= 1;
            if left == 0 {
                // TODO: Return descriptive error
                return Err(ServerError {});
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
