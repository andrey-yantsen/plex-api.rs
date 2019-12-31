use crate::{
    my_plex::{MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi,
};
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Privacy {
    opt_out_playback: bool,
    opt_out_library_stats: bool,
    domain: String,
    base_url: String,
    metrics: Vec<Metric>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Metric {
    event: String,
    status: String,
}

const PRIVACY_URL: &str = "api/v2/user/privacy";

impl MyPlexAccount {
    /// Returns current privacy settings, see [Privacy Preferences on plex.tv](https://www.plex.tv/about/privacy-legal/privacy-preferences/#opd).
    pub async fn get_privacy(&self) -> crate::Result<Privacy> {
        let response = self.get(PRIVACY_URL).await?;
        if response.status() == StatusCode::OK {
            let p: Privacy = response.json().await?;
            Ok(p)
        } else {
            let err: MyPlexApiErrorResponse = response.json().await?;
            Err(crate::error::PlexApiError::from(err))
        }
    }

    /// Changes privacy settings, see [Privacy Preferences on plex.tv](https://www.plex.tv/about/privacy-legal/privacy-preferences/#opd).
    pub async fn set_privacy(
        &self,
        opt_out_playback: bool,
        opt_out_library_stats: bool,
    ) -> crate::Result<()> {
        let mut params = HashMap::new();
        params.insert("optOutPlayback", if opt_out_playback { "1" } else { "0" });
        params.insert(
            "optOutLibraryStats",
            if opt_out_library_stats { "1" } else { "0" },
        );
        self.put_form(PRIVACY_URL, &params).await?;
        Ok(())
    }
}
