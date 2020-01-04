mod auth;
mod claim_token;
mod devices;
mod privacy;
mod resources;
mod users;
mod webhooks;

use crate::serde_helpers::bool_from_int;
use crate::{http::base_headers, HasPlexHeaders};
use chrono::DateTime;
use chrono::Utc;
use reqwest::header::HeaderMap;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct SubscriptionSummary {
    active: bool,
    #[serde(default)]
    subscribed_at: Option<DateTime<Utc>>,
    status: String,
    payment_service: String,
    plan: String,
    features: Vec<String>,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum AutoSelectSubtitleMode {
    ManuallySelected = 0,
    ShownWithForeignAudio = 1,
    AlwaysEnabled = 2,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Profile {
    auto_select_audio: bool,
    auto_select_subtitle: AutoSelectSubtitleMode,
    #[serde(deserialize_with = "bool_from_int")]
    default_subtitle_accessibility: bool,
    #[serde(deserialize_with = "bool_from_int")]
    default_subtitle_forced: bool,
    default_audio_language: String,
    default_subtitle_language: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Subscription {
    id: Option<i32>,
    mode: String,
    state: String,
    renews_at: Option<DateTime<Utc>>,
    ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    subscription_type: Option<String>,
    transfer: bool,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Service {
    identifier: String,
    endpoint: String,
    token: Option<String>,
    secret: Option<String>,
    status: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MyPlexAccount {
    id: i32,
    uuid: String,
    username: String,
    title: String,
    email: String,
    thumb: String,
    locale: Option<String>,
    email_only_auth: bool,
    has_password: bool,
    cloud_sync_device: Option<String>,
    auth_token: String,
    mailing_list_status: String,
    mailing_list_active: bool,
    scrobble_types: String,
    pin: String,
    subscription: SubscriptionSummary,
    subscription_description: String,
    restricted: bool,
    home: bool,
    guest: bool,
    queue_email: String,
    queue_uid: HashMap<String, String>,
    home_size: i32,
    max_home_size: i32,
    certificate_version: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    remember_expires_at: DateTime<Utc>,
    profile: Profile,
    entitlements: Vec<String>,
    roles: Vec<String>,
    subscriptions: Vec<Subscription>,
    services: Vec<Service>,
    protected: bool,
    country: String,
    home_admin: bool,
    trials: Vec<String>,
    ads_consent: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    ads_consent_set_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    ads_consent_reminder_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct MyPlexApiError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct MyPlexApiErrorResponse {
    errors: Vec<MyPlexApiError>,
}

pub trait HasMyPlexToken {
    fn get_auth_token(&self) -> &str;
    fn set_auth_token(&mut self, auth_token: &str);
}

impl<T> HasPlexHeaders for T
where
    T: HasMyPlexToken,
{
    fn headers(&self) -> HeaderMap {
        let mut headers = base_headers();

        if !self.get_auth_token().is_empty() {
            headers.insert("X-Plex-Token", self.get_auth_token().parse().unwrap());
        }

        headers.insert("Accept", "application/json".parse().unwrap());

        headers
    }
}

impl HasMyPlexToken for MyPlexAccount {
    /// Returns authentication token for current account.
    fn get_auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Sets authentication token for current account.
    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = String::from(auth_token);
    }
}

impl MyPlexAccount {
    /// Return username which was used to log in to MyPlex.
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}

impl crate::HasBaseUrl for MyPlexAccount {
    fn get_base_url(&self) -> String {
        String::from("https://plex.tv/")
    }
}

// TODO: Implement error conversion
impl From<MyPlexApiErrorResponse> for crate::error::PlexApiError {
    fn from(e: MyPlexApiErrorResponse) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}
