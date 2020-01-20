use crate::server::Server;
use crate::{CanMakeRequests, PlexApiError, Result};
use url::form_urlencoded;

const MYPLEX_ACCOUNT_URL: &str = "myplex/account";
const MYPLEX_CLAIM_URL: &str = "myplex/claim";

impl Server {
    pub async fn unclaim(&mut self) -> Result<()> {
        // Not sure whether it's true — I just wanted to use the macros
        required_server_version!(self, ">= 1.10", "Unclaim is available from 1.10 only");

        let response = self
            .prepare_query(MYPLEX_ACCOUNT_URL, reqwest::Method::DELETE)?
            .send()
            .await?;

        if response.status() == 200 {
            self.refresh().await
        } else {
            Err(PlexApiError::UnexpectedUnclaimError(response.text().await?))
        }
    }

    pub async fn claim(&mut self, claim_token: &str) -> Result<()> {
        // Not sure whether it's true — I just wanted to use the macros
        required_server_version!(self, ">= 1.10", "Claim is available from 1.10 only");

        if claim_token.is_empty() {
            return Err(PlexApiError::ClaimTokenEmpty);
        } else if !claim_token.starts_with("claim-") {
            return Err(PlexApiError::IncorrectClaimToken);
        }

        let params = form_urlencoded::Serializer::new(String::new())
            .append_pair("token", claim_token)
            .finish();
        let uri = MYPLEX_CLAIM_URL.to_owned() + "?" + &params;
        let response = self
            .prepare_query(&uri, reqwest::Method::POST)?
            .send()
            .await?;

        if response.status() == 200 {
            self.refresh().await
        } else {
            Err(PlexApiError::UnexpectedClaimError(response.text().await?))
        }
    }
}
