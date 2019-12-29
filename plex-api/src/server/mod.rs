use crate::{media_container::MediaContainer, HasBaseUrl, HasMyPlexToken};

mod connect;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Server {
    info: MediaContainer,
    url: String,
    auth_token: String,
}

impl HasMyPlexToken for Server {
    /// Returns authentication token for current server.
    fn get_auth_token(&self) -> String {
        self.auth_token.clone()
    }

    /// Sets authentication token for current server.
    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = String::from(auth_token);
    }
}

impl HasBaseUrl for Server {
    fn get_base_url(&self) -> String {
        self.url.clone()
    }
}