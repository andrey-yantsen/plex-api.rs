mod auth;
pub(crate) mod claim_token;
mod devices;
mod privacy;
mod resources;
mod users;
mod webhooks;

use crate::{http::base_headers, HasPlexHeaders, PlexApiError, Result};
use chrono::DateTime;
use chrono::Utc;
use reqwest::header::HeaderMap;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
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
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Profile {
    auto_select_audio: bool,
    auto_select_subtitle: AutoSelectSubtitleMode,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    default_subtitle_accessibility: bool,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_bool_from_anything")]
    default_subtitle_forced: bool,
    default_audio_language: String,
    default_subtitle_language: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct Subscription {
    id: Option<i32>,
    mode: String,
    state: String,
    renews_at: Option<DateTime<Utc>>,
    ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    subscription_type: Option<String>,
    transfer: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct Service {
    identifier: String,
    endpoint: String,
    token: Option<String>,
    secret: Option<String>,
    status: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
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
    pin: Option<String>,
    subscription: SubscriptionSummary,
    subscription_description: String,
    restricted: bool,
    home: bool,
    guest: bool,
    queue_email: Option<String>,
    queue_uid: Option<HashMap<String, String>>,
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
    ads_consent: Option<bool>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    ads_consent_set_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    ads_consent_reminder_at: Option<DateTime<Utc>>,
    anonymous: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct MyPlexApiError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct MyPlexApiErrorResponse {
    errors: Vec<MyPlexApiError>,
}

impl From<MyPlexApiErrorResponse> for PlexApiError {
    fn from(r: MyPlexApiErrorResponse) -> Self {
        let mut errors = vec![];
        for err in r.errors {
            errors.push(PlexApiError::MyPlexApiError {
                code: err.code,
                message: err.message,
            })
        }

        PlexApiError::MyPlexErrorResponse { errors }
    }
}

pub(crate) trait HasMyPlexToken {
    fn get_auth_token(&self) -> &str;
    fn set_auth_token(&mut self, auth_token: &str);
}

impl<T> HasPlexHeaders for T
where
    T: HasMyPlexToken,
{
    fn headers(&self) -> Result<HeaderMap> {
        let mut headers = base_headers()?;

        if !self.get_auth_token().is_empty() {
            headers.insert("X-Plex-Token", self.get_auth_token().parse()?);
        }

        headers.insert("Accept", "application/json".parse()?);

        Ok(headers)
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
    fn get_base_url(&self) -> &str {
        "https://plex.tv/"
    }
}
