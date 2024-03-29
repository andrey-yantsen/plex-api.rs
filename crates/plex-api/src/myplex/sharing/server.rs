use super::{friend::Friend, SharingFilter};
use crate::media_container::users::AllowTuners;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Settings {
    pub allow_channels: bool,
    pub allow_subtitle_admin: bool,
    pub allow_sync: bool,
    pub allow_camera_upload: bool,
    pub allow_tuners: AllowTuners,
    pub filter_movies: Option<SharingFilter>,
    pub filter_music: Option<SharingFilter>,
    pub filter_television: Option<SharingFilter>,
    pub filter_photos: Option<SharingFilter>,
    pub filter_all: Option<SharingFilter>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            allow_channels: Default::default(),
            allow_subtitle_admin: Default::default(),
            allow_sync: Default::default(),
            allow_tuners: AllowTuners::AllowLiveTvAndDvr,
            allow_camera_upload: Default::default(),
            filter_movies: Default::default(),
            filter_music: Default::default(),
            filter_television: Default::default(),
            filter_photos: Default::default(),
            filter_all: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SharedServer {
    pub id: u32,
    pub name: String,
    pub owner_id: u32,
    pub invited_id: u32,
    pub invited_email: Option<String>,
    pub server_id: u32,
    pub accepted: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub accepted_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub left_at: Option<OffsetDateTime>,
    pub owned: bool,
    pub invite_token: Option<String>,
    pub machine_identifier: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    pub num_libraries: u16,
    pub sharing_settings: Settings,
    pub libraries: Vec<crate::myplex::server::LibrarySection>,
    pub all_libraries: bool,
    pub invited: Option<Friend>,
}
