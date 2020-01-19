use crate::{
    CanMakeRequests, InternalHttpApi, PlexApiError, Result, Server, SettingsMediaContainer,
    SettingsMediaContainerOuter,
};
use url::form_urlencoded;

const SETTINGS_URL: &str = ":/prefs";

impl Server {
    pub async fn get_settings(&self) -> Result<SettingsMediaContainer> {
        let response = self.get(SETTINGS_URL).await?;
        if response.status() == reqwest::StatusCode::OK {
            Ok(SettingsMediaContainer::from(
                response.json::<SettingsMediaContainerOuter>().await?,
            ))
        } else {
            Err(PlexApiError::UnexpectedApiResponse(response.text().await?))
        }
    }

    pub async fn update_settings(&self, settings: &SettingsMediaContainer) -> Result<()> {
        let changed = settings.get_changed();
        if changed.is_empty() {
            Err(PlexApiError::NoChangedSettingsFound)
        } else {
            let mut params = form_urlencoded::Serializer::new(String::new());
            for (key, s) in changed {
                params.append_pair(key, &s.get_value().to_string());
            }

            let uri = SETTINGS_URL.to_owned() + "?" + &params.finish();

            let response = self
                .prepare_query(&uri, reqwest::Method::PUT)?
                .send()
                .await?;

            if response.status() == 200 {
                Ok(())
            } else {
                Err(PlexApiError::UnexpectedApiResponse(response.text().await?))
            }
        }
    }
}
