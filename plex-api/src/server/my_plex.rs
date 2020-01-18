use crate::server::Server;
use crate::{CanMakeRequests, PlexApiError, Result};

const MYPLEX_ACCOUNT_URL: &str = "myplex/account";

impl Server {
    pub async fn unclaim(&mut self) -> Result<()> {
        let response = self
            .prepare_query(MYPLEX_ACCOUNT_URL, reqwest::Method::DELETE)?
            .send()
            .await?;

        if response.status() == 200 {
            let new_server = if self.auth_token.is_empty() {
                Server::connect(&self.url.as_str().to_string()).await
            } else {
                Server::connect_auth(&self.url.as_str().to_string(), &self.auth_token).await
            };
            match new_server {
                Ok(srv) => {
                    *self = srv;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err(PlexApiError::UnexpectedUnclaimError(response.text().await?))
        }
    }
}
