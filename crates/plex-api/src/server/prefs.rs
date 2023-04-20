use crate::{
    media_container::{
        preferences::{Preferences as MediaContainerPreferences, Setting, Value},
        MediaContainerWrapper,
    },
    url::SERVER_PREFS,
    HttpClient, Result,
};
use std::mem::discriminant;

#[derive(Debug, Clone)]
pub struct Preferences<'a> {
    client: HttpClient,
    settings: Vec<Setting>,
    changed: Vec<&'a str>,
}

impl<'a> Preferences<'a> {
    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new<C: Into<HttpClient>>(client: C) -> Result<Preferences<'a>> {
        let client = client.into();
        let mc: MediaContainerWrapper<MediaContainerPreferences> =
            client.get(SERVER_PREFS).json().await?;

        Ok(Preferences {
            client,
            settings: mc.media_container.settings,
            changed: vec![],
        })
    }

    pub fn get(&self, key: &str) -> Option<&Setting> {
        self.settings.iter().find(|s| s.id == key)
    }

    pub fn all(&self) -> &[Setting] {
        &self.settings
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

    #[tracing::instrument(level = "debug")]
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

        self.client.put(uri).consume().await?;

        Ok(Preferences {
            client: self.client,
            settings: self.settings,
            changed: vec![],
        })
    }
}
