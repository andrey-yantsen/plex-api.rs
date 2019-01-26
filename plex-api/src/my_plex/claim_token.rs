use crate::my_plex::{MyPlexAccount, MyPlexError, Result};
use reqwest::StatusCode;

const CLAIM_TOKEN_URL: &str = "https://plex.tv/api/claim/token.json";

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct SuccessResponse {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct ErrorResponse {
    error: String,
}

impl MyPlexAccount {
    /// Returns a token which can help you to claim freshly installed Plex Server.
    ///
    /// See [https://www.plex.tv/claim](https://www.plex.tv/claim/),
    /// [docker:plexinc/pms-docker](https://hub.docker.com/r/plexinc/pms-docker/).
    pub fn get_claim_token(&self) -> Result<String> {
        let mut response = self.get(CLAIM_TOKEN_URL)?;
        match response.status() {
            StatusCode::OK => Ok((response.json::<SuccessResponse>()?).token.clone()),
            _ => Err(MyPlexError::from(response.json::<ErrorResponse>()?)),
        }
    }
}

// TODO: Implement error conversion
impl From<ErrorResponse> for MyPlexError {
    fn from(e: ErrorResponse) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}
