use crate::{media_container::ServerMediaContainer, HasBaseUrl, HasMyPlexToken};
use url::Url;

mod connect;

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct Server {
    info: ServerMediaContainer,
    url: Url,
    auth_token: String,
}

impl HasMyPlexToken for Server {
    /// Returns authentication token for current server.
    fn get_auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Sets authentication token for current server.
    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = String::from(auth_token);
    }
}

impl HasBaseUrl for Server {
    fn get_base_url(&self) -> &str {
        self.url.as_str()
    }
}
