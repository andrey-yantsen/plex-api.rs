use crate::SettingValue;
use reqwest::header::InvalidHeaderValue;
use std::collections::HashMap;
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum PlexApiError {
    #[error("Unable to perform a HTTP-request: {source}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("Unable to deserialize XML: {source}")]
    XmlDeserialiseError {
        #[from]
        source: quick_xml::DeError,
    },
    #[error("Unable to deserialize JSON: {source}")]
    JsonDeserealiseError {
        #[from]
        source: serde_json::Error,
    },
    #[error("RwLock was poisoned (reading)")]
    RwLockReadPoison,
    #[error("RwLock was poisoned (writing)")]
    RwLockWritePoison,
    #[error("Error while communicating with MyPlexApi: {errors:?}")]
    MyPlexErrorResponse { errors: Vec<PlexApiError> },
    #[error("Error occurred while communicating to MyPlex API: #{code} - {message}")]
    MyPlexApiError { code: i32, message: String },
    #[error("Unable to get the claim token: {0}")]
    FailedToGetClaimToken(String),
    #[error("Unable to parse request URL: {source}")]
    UrlParseError {
        #[from]
        source: url::ParseError,
    },
    #[error("Failed to delete the object because delete_url is unknown for it")]
    DeleteUrlIsNotProvided,
    #[error("Requested webhook not found ({url})")]
    WebhookNotFound { url: String },
    #[error("Received an unexpected response from API: {0}")]
    UnexpectedApiResponse(String),
    #[error("You've called update_settings(), but no settings were actually updated in provided container")]
    NoChangedSettingsFound,
    #[error("The device doesn't provide 'server' (but provides following: {provides:?})")]
    CurrentDeviceIsNotServer { provides: Vec<String> },
    #[error("Can't connect to the device without any connection methods provided")]
    EmptyConnectionsList,
    #[error("Can't connect to the device, occurred errors: {errors:?}")]
    ConnectionFailed { errors: HashMap<Url, PlexApiError> },
    #[error("Unknown setting requested (key={key}, known settings: {known}")]
    UnknownSettingRequested { key: String, known: String },
    #[error("SettingValue::Bool was expected, but provided another: {provided:?}")]
    ExpectedSettingValueBool { provided: SettingValue },
    #[error("SettingValue::Int was expected, but provided another: {provided:?}")]
    ExpectedSettingValueInt { provided: SettingValue },
    #[error("SettingValue::Text was expected, but provided another: {provided:?}")]
    ExpectedSettingValueText { provided: SettingValue },
    #[error("SettingValue::Double was expected, but provided another: {provided:?}")]
    ExpectedSettingValueDouble { provided: SettingValue },
    #[error("Unexpected error occurred during unclaiming the server: {0}")]
    UnexpectedUnclaimError(String),
    #[error("Unexpected error occurred during claiming the server: {0}")]
    UnexpectedClaimError(String),
    #[error("Claim token must be provided")]
    ClaimTokenEmpty,
    #[error("Incorrect claim token provided")]
    IncorrectClaimToken,
    #[error("Unable to parser header values")]
    InvalidHeaderValue {
        #[from]
        source: InvalidHeaderValue,
    },
    #[error("I/O-error occurred: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Unable to parse version requirement")]
    ReqSemverParsingError {
        #[from]
        source: semver::ReqParseError,
    },
    #[error("{message} (current server version: {current_version})")]
    ServerVersionLessThanRequired {
        message: String,
        required_version: String,
        current_version: String,
    },
}

impl<T> From<std::sync::PoisonError<RwLockWriteGuard<'_, T>>> for PlexApiError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        PlexApiError::RwLockWritePoison
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>> for PlexApiError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        PlexApiError::RwLockReadPoison
    }
}
