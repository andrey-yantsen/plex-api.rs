use crate::media_container::MediaContainer;
use crate::server::Server;
use crate::{base_headers, get_http_client};

impl Server {
    /// Establish a connection with the server server by provided url.
    ///
    /// This call will fail if anonymous access is denied.
    pub fn connect(url: &str) -> crate::Result<Self> {
        let mut response = get_http_client()?.get(url).headers(base_headers()).send()?;
        if response.status() == reqwest::StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            Ok(Server {
                info: mc,
                url: String::from(url),
            })
        } else {
            eprintln!("{:?}", response.text()?);
            Err(crate::error::PlexApiError {})
        }
    }

    /// Establish a connection with the server server by provided url and [`authentication token`].
    ///
    /// [`authentication token`]: struct.MyPlexAccount.html#method.get_auth_token
    pub fn login(url: &str, auth_token: &str) -> crate::Result<Self> {
        let mut response = get_http_client()?
            .get(url)
            .headers(base_headers())
            .header("X-Plex-Token", auth_token)
            .send()?;
        if response.status() == reqwest::StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            Ok(Server {
                info: mc,
                url: String::from(url),
            })
        } else {
            eprintln!("{:?}", response.text()?);
            Err(crate::error::PlexApiError {})
        }
    }
}
