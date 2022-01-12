use crate::myplex::account::SubscriptionFeature;
use isahc::{AsyncBody, AsyncReadResponseExt, Response as HttpResponse};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authenticated client must be provided.")]
    ClientNotAuthenticated,
    #[error("Non-authenticated client must be provided.")]
    ClientAuthenticated,
    #[error("Unable to deserialize JSON: {source}.")]
    JsonDeserealiseError {
        #[from]
        source: serde_json::Error,
    },
    #[error("Unable to deserialize XML: {source}.")]
    XmlDeserealiseError {
        #[from]
        source: quick_xml::de::DeError,
    },
    #[error("{source}")]
    UrlencodingError {
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    #[error("{source}")]
    HttpError {
        #[from]
        source: http::Error,
    },
    #[error("{source}")]
    IsahcError {
        #[from]
        source: isahc::Error,
    },
    #[error("{source}")]
    StdIoError {
        #[from]
        source: std::io::Error,
    },
    #[error("Error while communicating with MyPlexApi: {errors:?}.")]
    MyPlexErrorResponse { errors: Vec<Self> },
    #[error("Error occurred while communicating to MyPlex API: #{code} - {message}.")]
    MyPlexApiError { code: i32, message: String },
    #[error("Failed to get claim token: {0}.")]
    FailedToGetClaimToken(String),
    #[error("Unexpected API response: HTTP {status_code}, content: {content}.")]
    UnexpectedApiResponse { status_code: u16, content: String },
    #[error("The requested webhook wasn't found: {0}.")]
    WebhookNotFound(String),
    #[error("The mandatory feature is not available: {0}.")]
    SubscriptionFeatureNotAvailable(SubscriptionFeature),
    #[error("OTP is required for the authentication.")]
    OtpRequired,
    #[error("OTP is provided, but no username/password.")]
    UselessOtp,
}

const PLEX_API_ERROR_CODE_AUTH_OTP_REQUIRED: i32 = 1029;

impl Error {
    pub async fn from_response(mut response: HttpResponse<AsyncBody>) -> Self {
        let status_code = response.status().as_u16();
        let response_body = match response.text().await {
            Ok(body) => body,
            Err(err) => {
                return err.into();
            }
        };
        let err = serde_json::from_str::<MyPlexApiErrorResponse>(&response_body);
        match err {
            Ok(r) => {
                if r.errors.len() == 1 && r.errors[0].code == PLEX_API_ERROR_CODE_AUTH_OTP_REQUIRED
                {
                    Self::OtpRequired
                } else {
                    r.into()
                }
            }
            Err(_) => Self::UnexpectedApiResponse {
                status_code,
                content: response_body,
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
struct MyPlexApiError {
    code: i32,
    message: String,

    /// HTTP status code for the error. Not used, keeping it here to reflect the complete API response.
    #[allow(dead_code)]
    status: u16,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub(crate) struct MyPlexApiErrorResponse {
    errors: Vec<MyPlexApiError>,
}

impl From<MyPlexApiError> for Error {
    fn from(error: MyPlexApiError) -> Self {
        Self::MyPlexApiError {
            code: error.code,
            message: error.message,
        }
    }
}

impl From<MyPlexApiErrorResponse> for Error {
    fn from(r: MyPlexApiErrorResponse) -> Self {
        Self::MyPlexErrorResponse {
            errors: r.errors.into_iter().map(|e| e.into()).collect(),
        }
    }
}
