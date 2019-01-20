use crate::media_container::MediaContainer;
use crate::server::{Result, Server, ServerError};
use crate::{base_headers, get_http_client};

impl Server {
    pub fn connect(url: &str) -> Result<Self> {
        dbg!(url);
        let mut response = get_http_client()?.get(url).headers(base_headers()).send()?;
        if response.status() == reqwest::StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            Ok(Server {
                info: mc,
                url: String::from(url),
            })
        } else {
            eprintln!("{:?}", response.text()?);
            Err(ServerError {})
        }
    }

    pub fn login(url: &str, auth_token: &str) -> Result<Self> {
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
            Err(ServerError {})
        }
    }
}
