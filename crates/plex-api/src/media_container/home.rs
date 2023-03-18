use serde::Deserialize;

use crate::RestrictionProfile;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct User {
    #[serde(rename = "@id")]
    pub id: u64,
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(rename = "@admin")]
    pub admin: bool,
    #[serde(rename = "@guest")]
    pub guest: bool,
    #[serde(rename = "@restricted")]
    pub restricted: bool,
    #[serde(rename = "@restrictionProfile")]
    pub restriction_profile: Option<RestrictionProfile>,
    #[serde(rename = "@hasPassword")]
    pub has_password: bool,
    #[serde(rename = "@protected")]
    pub protected: bool,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@username")]
    pub username: Option<String>,
    #[serde(rename = "@email")]
    pub email: Option<String>,
    #[serde(with = "http_serde::uri", rename = "@thumb")]
    pub thumb: http::Uri,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct UsersResponse {
    #[serde(rename = "User")]
    pub users: Vec<User>,

    #[serde(rename = "@friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "@identifier")]
    pub identifier: String,
    #[serde(rename = "@machineIdentifier")]
    pub machine_identifier: String,
    #[serde(rename = "@totalSize")]
    pub total_size: u16,
    #[serde(rename = "@size")]
    pub size: u16,
    #[serde(rename = "@guestUserID")]
    pub guest_user_id: u64,
}
