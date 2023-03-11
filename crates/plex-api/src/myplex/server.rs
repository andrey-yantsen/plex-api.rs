use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct LibrarySection {
    pub id: u32,
    pub key: u32,
    pub title: String,
    pub r#type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ServerInfo {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub version: String,
    pub scheme: String,
    pub synced: bool,
    pub owned: bool,
    pub local_addresses: String,
    pub machine_identifier: String,
    #[serde(with = "time::serde::timestamp")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
    pub library_sections: Vec<LibrarySection>,
}
