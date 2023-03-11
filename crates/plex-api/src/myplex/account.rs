use secrecy::SecretString;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use serde_with::{json::JsonString, serde_as};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSummary {
    pub active: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub subscribed_at: Option<OffsetDateTime>,
    pub status: SubscriptionStatus,
    pub payment_service: Option<String>,
    pub plan: Option<String>,
    pub features: Vec<crate::media_container::server::Feature>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Canceled,
    PendingCancellation,
    Ended,
    Lapsed,
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum AutoSelectSubtitleMode {
    ManuallySelected = 0,
    ShownWithForeignAudio = 1,
    AlwaysEnabled = 2,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub auto_select_audio: bool,
    pub auto_select_subtitle: AutoSelectSubtitleMode,
    #[serde(deserialize_with = "bool_from_int")]
    pub default_subtitle_accessibility: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub default_subtitle_forced: bool,
    pub default_audio_language: Option<String>,
    pub default_subtitle_language: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,
    pub mode: String,
    pub state: String,
    pub renews_at: Option<OffsetDateTime>,
    pub ends_at: Option<OffsetDateTime>,
    pub r#type: Option<String>,
    pub transfer: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Service {
    pub identifier: String,
    pub endpoint: String,
    pub token: Option<SecretString>,
    pub secret: Option<SecretString>,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MyPlexAccount {
    pub id: u64,
    pub uuid: String,
    pub username: String,
    pub friendly_name: String,
    pub confirmed: bool,
    pub title: String,
    pub email: String,
    pub thumb: String,
    pub locale: Option<String>,
    pub email_only_auth: bool,
    pub has_password: bool,
    pub cloud_sync_device: Option<String>,
    pub auth_token: SecretString,
    pub mailing_list_status: Option<String>,
    pub mailing_list_active: bool,
    pub scrobble_types: String,
    pub pin: Option<String>,
    pub subscription: SubscriptionSummary,
    pub subscription_description: Option<String>,
    pub restricted: bool,
    pub home: bool,
    pub guest: bool,
    pub queue_email: Option<String>,
    pub queue_uid: Option<HashMap<String, String>>,
    pub home_size: i32,
    pub max_home_size: i32,
    #[serde(with = "time::serde::timestamp::option")]
    pub remember_expires_at: Option<OffsetDateTime>,
    pub profile: Profile,
    pub entitlements: Vec<String>,
    pub roles: Option<Vec<String>>,
    pub services: Vec<Service>,
    pub protected: bool,
    pub country: String,
    pub home_admin: bool,
    pub ads_consent: Option<bool>,
    #[serde(with = "time::serde::timestamp::option")]
    pub ads_consent_set_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    pub ads_consent_reminder_at: Option<OffsetDateTime>,
    pub anonymous: Option<bool>,
    pub experimental_features: bool,
    pub two_factor_enabled: bool,
    pub backup_codes_created: bool,
    #[serde(with = "time::serde::timestamp")]
    pub joined_at: OffsetDateTime,

    pub restriction_profile: Option<RestrictionProfile>,

    // Some managed guest users don't have subscription info
    pub subscriptions: Option<Vec<Subscription>>,
    pub past_subscriptions: Option<Vec<Subscription>>,
    pub trials: Option<Vec<Subscription>>,

    // TODO: The next field was met only once
    pub settings: Option<Vec<Settings>>,

    // TODO: I don't have a valid example with the data yet for the following fields
    pub custom_restrictions: Option<CustomRestrictions>,
    pub providers: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct CustomRestrictions {
    pub all: Option<bool>,
    pub movies: Option<bool>,
    pub music: Option<bool>,
    pub photos: Option<bool>,
    pub television: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "id", rename_all = "camelCase")]
pub enum Settings {
    Experience(ExperienceSettingsContainer),
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSettingsContainer {
    pub hidden: bool,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,

    #[serde(flatten)]
    pub settings: ExperienceSettingsFormat,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum ExperienceSettingsFormat {
    Json(#[serde_as(as = "JsonString")] ExperienceSettings),
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSettings {
    pub auto_home_hubs_enabled: bool,
    pub auto_pinned_providers: Vec<String>,
    pub schema_version: i32,
    pub home_settings: ExperienceHomeSettings,
    pub sidebar_settings: ExperienceSidebarSettings,
    pub reminders: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceHomeSettings {
    pub settings_key: String,
    pub hubs: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSidebarSettings {
    pub has_completed_setup: bool,
    pub pinned_sources: Vec<SidebarSource>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SidebarSource {
    pub key: String,
    pub source_type: String,
    pub machine_identifier: String,
    pub provider_identifier: String,
    #[serde(rename = "directoryID")]
    pub directory_id: String,
    pub directory_icon: String,
    pub title: String,
    pub server_friendly_name: String,
    pub provider_source_title: String,
    pub is_cloud: bool,
    pub is_full_owned_server: bool,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionProfile {
    LittleKid,
    OlderKid,
    Teen,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Unexpected};

    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
