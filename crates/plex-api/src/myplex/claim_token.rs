use crate::{http_client::HttpClient, url::MYPLEX_CLAIM_TOKEN_PATH, Error, Result};
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

/// The lifetime of the token is not provided by the API, it's hardcoded to 4 minutes.
const CLAIM_TOKEN_LIFETIME_SECONDS: i64 = 60 * 4;

#[derive(Clone, Debug)]
pub struct ClaimToken {
    token: String,
    expires: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct SuccessResponse {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct ErrorResponse {
    error: String,
}

impl ClaimToken {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn new(client: &HttpClient) -> Result<Self> {
        let mut response = client.get(MYPLEX_CLAIM_TOKEN_PATH).send().await?;
        match response.status() {
            StatusCode::OK => {
                let token = response.json::<SuccessResponse>().await?.token;
                Ok(Self {
                    token,
                    expires: OffsetDateTime::now_utc()
                        + Duration::seconds(CLAIM_TOKEN_LIFETIME_SECONDS),
                })
            }
            _ => {
                let error: ErrorResponse = response.json().await?;
                Err(Error::FailedToGetClaimToken(error.error))
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires < OffsetDateTime::now_utc()
    }
}

impl ToString for ClaimToken {
    fn to_string(&self) -> String {
        self.token.clone()
    }
}
