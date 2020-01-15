use crate::{my_plex::MyPlexAccount, InternalHttpApi, PlexApiError};
use reqwest::StatusCode;

const CLAIM_TOKEN_URL: &str = "api/claim/token.json";

#[derive(Serialize, Deserialize)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct SuccessResponse {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct ErrorResponse {
    error: String,
}

impl MyPlexAccount {
    /// Returns a token which can help you to claim freshly installed Plex Server.
    ///
    /// See [https://www.plex.tv/claim](https://www.plex.tv/claim/),
    /// [docker:plexinc/pms-docker](https://hub.docker.com/r/plexinc/pms-docker/).
    pub async fn get_claim_token(&self) -> crate::Result<String> {
        let response = self.get(CLAIM_TOKEN_URL).await?;
        match response.status() {
            StatusCode::OK => Ok((response.json::<SuccessResponse>()).await?.token),
            _ => {
                let error = response.json::<ErrorResponse>().await?;
                Err(PlexApiError::FailedToGetClaimToken(error.error))
            }
        }
    }
}
