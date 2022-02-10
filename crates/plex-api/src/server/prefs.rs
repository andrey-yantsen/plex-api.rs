use std::mem::discriminant;

use crate::media_container::preferences::{
    Preferences as MediaContainerPreferences, Setting, Value,
};
use crate::url::SERVER_PREFS;
use crate::{media_container::MediaContainerWrapper, HttpClient, Result};
use http::StatusCode;
use isahc::AsyncReadResponseExt;

#[derive(Debug, Clone)]
pub struct Preferences<'a> {
    client: &'a HttpClient,
    settings: Vec<Setting>,
    changed: Vec<&'a str>,
}

impl<'a> Preferences<'a> {
    pub async fn new(client: &'a HttpClient) -> Result<Preferences<'a>> {
        let mut response = client
            .get(SERVER_PREFS)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let mc: MediaContainerWrapper<MediaContainerPreferences> = response.json().await?;
            Ok(Preferences {
                client,
                settings: mc.media_container.settings,
                changed: vec![],
            })
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }

    pub fn get(&self, key: &str) -> Option<&Setting> {
        self.settings.iter().find(|s| s.id == key)
    }

    pub fn set(&mut self, key: &'a str, value: Value) -> Result<&mut Self> {
        if let Some(setting) = self.settings.iter_mut().find(|s| s.id == key) {
            if discriminant(&setting.value) == discriminant(&value) {
                setting.value = value;
                self.changed.push(key);
                Ok(self)
            } else {
                Err(crate::Error::IncompatibleSettingValues)
            }
        } else {
            Err(crate::Error::RequestedSettingNotFound(key.to_string()))
        }
    }

    pub async fn commit(self) -> Result<Preferences<'a>> {
        if self.changed.is_empty() {
            return Ok(self);
        }

        let mut params = vec![];
        for s in self
            .settings
            .iter()
            .filter(|s| self.changed.contains(&s.id.as_str()))
        {
            params.push((s.id.as_str(), s.value.to_string()));
        }

        let uri = format!("{}?{}", SERVER_PREFS, serde_urlencoded::to_string(params)?);

        let response = self
            .client
            .put(uri)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            Ok(Preferences {
                client: self.client,
                settings: self.settings,
                changed: vec![],
            })
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }
}
