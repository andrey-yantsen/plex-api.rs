use crate::server::Server;
use crate::{CanMakeRequests, InternalHttpApi, PlexApiError, Result};
use url::form_urlencoded;

const MYPLEX_ACCOUNT_URL: &str = "myplex/account";
const MYPLEX_CLAIM_URL: &str = "myplex/claim";

impl Server {
    pub async fn unclaim(&mut self) -> Result<()> {
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
        if claim_token.is_empty() {
            return Err(PlexApiError::ClaimTokenEmpty);
        } else if !claim_token.starts_with("claim-") {
            return Err(PlexApiError::IncorrectClaimToken);
        }

        let params = form_urlencoded::Serializer::new(String::new())
            .append_pair("token", claim_token)
            .finish();
        let uri = MYPLEX_CLAIM_URL.to_owned() + "?" + &params;
        let response = self.get(&uri).await?;

        if response.status() == 200 {
            self.refresh().await
        } else {
            Err(PlexApiError::UnexpectedClaimError(response.text().await?))
        }
    }

    async fn refresh(&mut self) -> Result<()> {
        let new_server = Server::connect(&self.url.as_str().to_string(), &self.auth_token).await;
        match new_server {
            Ok(srv) => {
                *self = srv;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
