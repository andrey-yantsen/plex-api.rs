use crate::{
    CanMakeRequests, HasBaseUrl, InternalHttpApi, PlexApiError, Result, Server,
    SettingsMediaContainer, SettingsMediaContainerOuter,
};
use url::Url;

const SETTINGS_URL: &str = ":/prefs";

impl Server {
    pub async fn get_settings(&self) -> Result<SettingsMediaContainer> {
        let response = self.get(SETTINGS_URL).await?;
        if response.status() == reqwest::StatusCode::OK {
            Ok(SettingsMediaContainer::from(
                response.json::<SettingsMediaContainerOuter>().await?,
            ))
        } else {
            // TODO: Error handling
            eprintln!("{:?}", response.text().await?);
            Err(crate::error::PlexApiError {})
        }
    }

    pub async fn update_settings(&self, settings: &SettingsMediaContainer) -> Result<()> {
        let changed = settings.get_changed();
        if changed.is_empty() {
            Err(PlexApiError {})
        } else {
            let mut uri = Url::parse(&(self.get_base_url().to_owned() + SETTINGS_URL)).unwrap();

            {
                let mut pairs = uri.query_pairs_mut();

                for (key, s) in changed {
                    pairs.append_pair(key, &s.get_value().to_string());
                }
            }

            let response = self
                .prepare_query(uri, reqwest::Method::PUT)?
                .send()
                .await?;

            if response.status() == 200 {
                Ok(())
            } else {
                unimplemented!("{}", response.text().await?);
            }
        }
    }
}
