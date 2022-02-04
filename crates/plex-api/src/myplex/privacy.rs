use crate::{http_client::HttpClient, url::MYPLEX_PRIVACY_PATH, Error};
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct PrivacyApiResponse {
    pub opt_out_playback: bool,
    pub opt_out_library_stats: bool,
    pub domain: String,
    pub base_url: String,
    pub metrics: Vec<Metric>,
}

#[derive(Debug)]
pub struct Privacy {
    pub opt_out_playback: bool,
    pub opt_out_library_stats: bool,

    client: Arc<HttpClient>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[allow(dead_code)]
struct Metric {
    pub event: String,
    pub status: String,
}

impl Privacy {
    /// Returns current privacy settings, see [Privacy Preferences on plex.tv](https://www.plex.tv/about/privacy-legal/privacy-preferences/#opd).
    pub async fn new(client: Arc<HttpClient>) -> crate::Result<Self> {
        let mut response = client.get(MYPLEX_PRIVACY_PATH).send().await?;
        if response.status() == StatusCode::OK {
            let p: PrivacyApiResponse = response.json().await?;
            Ok(Self {
                opt_out_playback: p.opt_out_playback,
                opt_out_library_stats: p.opt_out_library_stats,
                client,
            })
        } else {
            Err(Error::from_response(response).await)
        }
    }

    /// Changes privacy settings, see [Privacy Preferences on plex.tv](https://www.plex.tv/about/privacy-legal/privacy-preferences/#opd).
    pub async fn update(
        &mut self,
        opt_out_playback: bool,
        opt_out_library_stats: bool,
    ) -> crate::Result<()> {
        let params = vec![
            ("optOutPlayback", if opt_out_playback { "1" } else { "0" }),
            (
                "optOutLibraryStats",
                if opt_out_library_stats { "1" } else { "0" },
            ),
        ];
        let response = self
            .client
            .put(MYPLEX_PRIVACY_PATH)
            .form(&params)?
            .send()
            .await?;
        if response.status() == StatusCode::NO_CONTENT {
            self.opt_out_library_stats = opt_out_library_stats;
            self.opt_out_playback = opt_out_playback;
            Ok(())
        } else {
            Err(Error::from_response(response).await)
        }
    }
}
