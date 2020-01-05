use crate::MediaContainer;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UsersMediaContainer {
    machine_identifier: String,
    #[serde(rename = "User")]
    users: Option<Vec<User>>,
    #[serde(flatten)]
    media_container: MediaContainer,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    title: String,
    thumb: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    protected: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    home: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_sync: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_camera_upload: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_channels: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_tuners: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    allow_subtitle_admin: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    restricted: bool,
    filter_all: String,
    filter_movies: String,
    filter_music: String,
    filter_photos: String,
    filter_television: String,
    #[serde(rename = "Server")]
    servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Server {
    id: u32,
    server_id: u32,
    machine_identifier: String,
    name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    last_seen_at: DateTime<Utc>,
    num_libraries: u32,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    all_libraries: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    owned: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    pending: bool,
}

impl UsersMediaContainer {
    pub fn get_media_container(self) -> MediaContainer {
        self.media_container
    }
    pub fn get_users(self) -> Option<Vec<User>> {
        self.users
    }
}
