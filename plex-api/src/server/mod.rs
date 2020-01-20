/// Compare current server's version with the requirements (should be formatter as proper semver
/// requirement, e.g. ">= 1.10" or "<= 1.16"). If requirement is not met, `return` would be called
/// with error `PlexApiError::ServerVersionLessThanRequired`.
macro_rules! required_server_version {
    ($srv:ident, $version:literal, $error:literal) => {
        use semver::VersionReq;
        match VersionReq::parse($version) {
            Ok(req) => {
                if !req.matches($srv.get_version()) {
                    return Err(crate::error::PlexApiError::ServerVersionLessThanRequired {
                        message: $error.to_string(),
                        required_version: $version.to_string(),
                        current_version: $srv.get_version().to_string(),
                    });
                }
            }
            Err(e) => return Err(core::convert::From::from(e)),
        }
    };
}

mod connect;
mod my_plex;
mod settings;

use crate::{media_container::ServerMediaContainer, HasBaseUrl, HasMyPlexToken};
use semver::Version;
use url::Url;

#[derive(Deserialize, Debug)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct Server {
    info: ServerMediaContainer,
    url: Url,
    auth_token: String,
}

impl Server {
    pub const fn get_version(&self) -> &Version {
        self.info.get_version()
    }
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
