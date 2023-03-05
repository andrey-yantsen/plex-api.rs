use super::MediaContainer;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UsersMediaContainer {
    pub machine_identifier: String,
    #[serde(rename = "User", default)]
    pub users: Vec<User>,
    #[serde(flatten)]
    pub media_container: MediaContainer,
}

#[derive(Debug, Deserialize_repr, Clone, Serialize_repr)]
#[repr(u8)]
pub enum AllowTuners {
    None = 0,
    AllowLiveTv = 1,
    AllowLiveTvAndDvr = 2,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub title: String,
    pub thumb: String,
    pub protected: bool,
    pub home: bool,
    pub allow_sync: bool,
    pub allow_camera_upload: bool,
    pub allow_channels: bool,
    pub allow_tuners: AllowTuners,
    pub allow_subtitle_admin: bool,
    pub restricted: bool,
    pub filter_all: String,
    pub filter_movies: String,
    pub filter_music: String,
    pub filter_photos: String,
    pub filter_television: String,
    #[serde(rename = "Server")]
    pub servers: Option<Vec<UserServer>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UserServer {
    pub id: u32,
    pub server_id: u32,
    pub machine_identifier: String,
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub last_seen_at: OffsetDateTime,
    pub num_libraries: u32,
    pub all_libraries: bool,
    pub owned: bool,
    pub pending: bool,
}
