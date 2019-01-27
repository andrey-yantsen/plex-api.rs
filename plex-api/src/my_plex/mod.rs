mod auth;
mod claim_token;
mod devices;
mod privacy;
mod resources;
mod users;
mod webhooks;

use crate::serde_helpers::bool_from_int;
use crate::{base_headers, get_http_client, HasPlexHeaders};
use chrono::DateTime;
use chrono::Utc;
use reqwest::{header::HeaderMap, IntoUrl};
use serde::Serialize;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct SubscriptionSummary {
    active: bool,
    #[serde(rename = "subscribedAt", default)]
    subscribed_at: Option<DateTime<Utc>>,
    status: String,
    #[serde(rename = "paymentService")]
    payment_service: String,
    plan: String,
    features: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Profile {
    #[serde(rename = "autoSelectAudio")]
    auto_select_audio: bool,
    #[serde(rename = "autoSelectSubtitle", deserialize_with = "bool_from_int")]
    auto_select_subtitle: bool,
    #[serde(
        rename = "defaultSubtitleAccessibility",
        deserialize_with = "bool_from_int"
    )]
    default_subtitle_accessibility: bool,
    #[serde(rename = "defaultSubtitleForced", deserialize_with = "bool_from_int")]
    default_subtitle_forced: bool,
    #[serde(rename = "defaultAudioLanguage")]
    default_audio_language: String,
    #[serde(rename = "defaultSubtitleLanguage")]
    default_subtitle_language: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Subscription {
    id: Option<i32>,
    mode: String,
    state: String,
    #[serde(rename = "renewsAt")]
    renews_at: Option<DateTime<Utc>>,
    #[serde(rename = "endsAt")]
    ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    subscription_type: Option<String>,
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
pub struct MyPlexAccount {
    id: i32,
    uuid: String,
    username: String,
    title: String,
    email: String,
    thumb: String,
    locale: Option<String>,
    #[serde(rename = "emailOnlyAuth")]
    email_only_auth: bool,
    #[serde(rename = "hasPassword")]
    has_password: bool,
    #[serde(rename = "cloudSyncDevice")]
    cloud_sync_device: Option<String>,
    #[serde(rename = "authToken")]
    auth_token: String,
    #[serde(rename = "mailingListStatus")]
    mailing_list_status: String,
    #[serde(rename = "mailingListActive")]
    mailing_list_active: bool,
    #[serde(rename = "scrobbleTypes")]
    scrobble_types: String,
    pin: String,
    subscription: SubscriptionSummary,
    #[serde(rename = "subscriptionDescription")]
    subscription_description: String,
    restricted: bool,
    home: bool,
    guest: bool,
    #[serde(rename = "queueEmail")]
    queue_email: String,
    #[serde(rename = "queueUid")]
    queue_uid: String,
    #[serde(rename = "homeSize")]
    home_size: i32,
    #[serde(rename = "maxHomeSize")]
    max_home_size: i32,
    #[serde(rename = "certificateVersion")]
    certificate_version: i32,
    #[serde(rename = "rememberExpiresAt", with = "chrono::serde::ts_seconds")]
    remember_expires_at: DateTime<Utc>,
    profile: Profile,
    entitlements: Vec<String>,
    roles: Vec<String>,
    subscriptions: Vec<Subscription>,
    services: Vec<Service>,
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
    fn get_auth_token(&self) -> String;
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
    fn get_auth_token(&self) -> String {
        self.auth_token.clone()
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

    fn get<U: IntoUrl>(&self, url: U) -> crate::Result<reqwest::Response> {
        get_http_client()?
            .get(url)
            .headers(self.headers())
            .send()
            .map_err(From::from)
    }

    fn post_form<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        params: &T,
    ) -> crate::Result<reqwest::Response> {
        get_http_client()?
            .post(url)
            .form(params)
            .headers(self.headers())
            .send()
            .map_err(From::from)
    }

    fn put_form<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        params: &T,
    ) -> crate::Result<reqwest::Response> {
        get_http_client()?
            .put(url)
            .form(params)
            .headers(self.headers())
            .send()
            .map_err(From::from)
    }
}

// TODO: Implement error conversion
impl From<MyPlexApiErrorResponse> for crate::error::PlexApiError {
    fn from(e: MyPlexApiErrorResponse) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}
