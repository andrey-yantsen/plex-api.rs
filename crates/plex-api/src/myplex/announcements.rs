use crate::{url::MYPLEX_ANNOUNCEMENTS, HttpClient, Result};
use http::Uri;
use serde::Deserialize;
use time::OffsetDateTime;

pub struct AnnouncementsManager {
    client: HttpClient,
    container: AnnouncementsContainer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementStyle {
    Info,
    Priority,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Announcement {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@content")]
    pub content: String,
    #[serde(rename = "@plainContent")]
    pub plain_content: String,

    #[serde(with = "http_serde::uri", rename = "@imageUrl")]
    pub image_url: Uri,
    #[serde(rename = "@style")]
    pub style: AnnouncementStyle,
    #[serde(rename = "@url")]
    pub url: String, // Not using Uri here since the url might be empty
    #[serde(rename = "@expireAt", with = "time::serde::timestamp")]
    pub expire_at: OffsetDateTime,
    #[serde(rename = "@notifyAt", with = "time::serde::timestamp")]
    pub notify_at: OffsetDateTime,
    #[serde(rename = "@read")]
    pub read: bool,
    #[serde(skip)]
    client: Option<HttpClient>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct AnnouncementsContainer {
    #[serde(rename = "@friendlyName")]
    _friendly_name: String,
    #[serde(rename = "@identifier")]
    _identifier: String,
    #[serde(rename = "@machineIdentifier")]
    _machine_identifier: String,
    #[serde(rename = "@totalSize")]
    _total_size: u8,
    #[serde(rename = "@size")]
    _size: u8,
    #[serde(rename = "@title1")]
    _title1: String,
    #[serde(rename = "@viewGroup")]
    _view_group: String,
    #[serde(rename = "@content")]
    _content: String,

    #[serde(rename = "Announcement", default)]
    announcements: Vec<Announcement>,
}

impl AnnouncementsManager {
    async fn load_container(client: &HttpClient) -> Result<AnnouncementsContainer> {
        let mut ret: AnnouncementsContainer = client.get(MYPLEX_ANNOUNCEMENTS).xml().await?;
        for announcement in &mut ret.announcements {
            announcement.client = Some(client.clone());
        }

        Ok(ret)
    }

    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new(client: HttpClient) -> Result<Self> {
        let container = Self::load_container(&client).await?;

        Ok(Self { client, container })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    /// Refresh the available announcement list and their status
    pub async fn refresh(&mut self) -> Result<()> {
        self.container = Self::load_container(&self.client).await?;
        Ok(())
    }

    /// Return the available announcements list
    pub fn announcements(&self) -> &Vec<Announcement> {
        &self.container.announcements
    }

    /// Return the available announcements list for further modifications (e.g. marking as read/unread)
    pub fn announcements_mut(&mut self) -> &mut Vec<Announcement> {
        &mut self.container.announcements
    }
}

#[derive(Deserialize)]
struct AnnouncementApiResponce {
    #[serde(rename = "@code")]
    code: u16,
    #[serde(rename = "@status")]
    status: String,
}

impl From<AnnouncementApiResponce> for crate::Error {
    fn from(error: AnnouncementApiResponce) -> Self {
        Self::MyPlexApiError {
            code: i32::from(error.code),
            message: error.status,
        }
    }
}

impl Announcement {
    /// Mark the announcement as read
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read(&mut self) -> Result<()> {
        let response: AnnouncementApiResponce = self
            .client
            .as_ref()
            .unwrap()
            .put(format!("{MYPLEX_ANNOUNCEMENTS}/{}?read=1", self.id))
            .xml()
            .await?;

        if response.code == 200 {
            Ok(())
        } else {
            Err(response.into())
        }
    }

    /// Mark the announcement as unread
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn unread(&mut self) -> Result<()> {
        let response: AnnouncementApiResponce = self
            .client
            .as_ref()
            .unwrap()
            .put(format!("{MYPLEX_ANNOUNCEMENTS}/{}?read=0", self.id))
            .xml()
            .await?;

        if response.code == 200 {
            Ok(())
        } else {
            Err(response.into())
        }
    }
}
