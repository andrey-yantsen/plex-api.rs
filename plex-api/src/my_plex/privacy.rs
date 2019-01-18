use crate::my_plex::{MyPlex, MyPlexError, Result};
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Privacy {
    #[serde(rename = "optOutPlayback")]
    opt_out_playback: bool,
    #[serde(rename = "optOutLibraryStats")]
    opt_out_library_stats: bool,
    domain: String,
    #[serde(rename = "baseUrl")]
    base_url: String,
    metrics: Vec<Metric>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Metric {
    event: String,
    status: String,
}

const PRIVACY_URL: &str = "https://plex.tv/api/v2/user/privacy";

impl MyPlex {
    pub fn get_privacy(&self) -> Result<Privacy> {
        let mut response = self.get(PRIVACY_URL)?;
        if response.status() == StatusCode::OK {
            let p: Privacy = response.json()?;
            Ok(p)
        } else {
            Err(MyPlexError {})
        }
    }

    pub fn set_privacy(&self, opt_out_playback: bool, opt_out_library_stats: bool) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("optOutPlayback", if opt_out_playback { "1" } else { "0" });
        params.insert(
            "optOutLibraryStats",
            if opt_out_library_stats { "1" } else { "0" },
        );
        self.put_form(PRIVACY_URL, &params)?;
        Ok(())
    }
}
