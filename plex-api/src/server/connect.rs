use crate::http::{base_headers, get_http_client};
use crate::media_container::ServerMediaContainer;
use crate::server::Server;
use crate::PlexApiError;

impl Server {
    /// Establish a connection with the server server by provided url and [`authentication token`].
    /// If you need an anonymous connection — just provide an empty token.
    ///
    /// This call will fail if if token is empty and anonymous access is denied.
    pub async fn connect<U: reqwest::IntoUrl + crate::AsStr + Send>(url: U, auth_token: &str) -> crate::Result<Self> {
        let response = get_http_client()?
            .get(url.as_str())
            .headers(base_headers()?)
            .header("X-Plex-Token", auth_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            let mc: ServerMediaContainer =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            Ok(Server {
                info: mc,
                url: url.into_url()?,
                auth_token: String::from(auth_token),
            })
        } else {
            Err(PlexApiError::UnexpectedApiResponse(response.text().await?))
        }
    }
}
