pub mod library;
pub mod prefs;

use self::{
    library::{metadata_items, Library},
    prefs::Preferences,
};
#[cfg(not(feature = "tests_deny_unknown_fields"))]
use crate::media_container::server::library::LibraryType;
use crate::{
    http_client::HttpClient,
    media_container::{
        server::{library::ContentDirectory, MediaProviderFeature, Server as ServerMediaContainer},
        MediaContainerWrapper,
    },
    myplex::MyPlex,
    url::{SERVER_MEDIA_PROVIDERS, SERVER_MYPLEX_ACCOUNT, SERVER_MYPLEX_CLAIM},
    Error, HttpClientBuilder, Item, Result,
};
use core::convert::TryFrom;
use http::{StatusCode, Uri};
use isahc::AsyncReadResponseExt;

#[derive(Debug, Clone)]
pub struct Server {
    client: HttpClient,
    myplex_api_url: Uri,
    pub media_container: ServerMediaContainer,
}

impl Server {
    async fn build(client: HttpClient, myplex_api_url: Uri) -> Result<Self> {
        let media_container_wrapper: MediaContainerWrapper<ServerMediaContainer> =
            client.get(SERVER_MEDIA_PROVIDERS).json().await?;

        Ok(Self {
            media_container: media_container_wrapper.media_container,
            client,
            myplex_api_url,
        })
    }

    pub async fn new<U>(url: U, client: HttpClient) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let myplex_api_url = client.api_url.clone();
        Self::build(
            HttpClientBuilder::from(client).set_api_url(url).build()?,
            myplex_api_url,
        )
        .await
    }

    fn content(&self) -> Option<&Vec<ContentDirectory>> {
        if let Some(provider) = self
            .media_container
            .media_providers
            .iter()
            .find(|p| p.identifier == "com.plexapp.plugins.library")
        {
            for feature in &provider.features {
                if let MediaProviderFeature::Content {
                    key: _,
                    ref directory,
                } = feature
                {
                    return Some(directory);
                }
            }
        }

        None
    }

    pub fn libraries(&self) -> Vec<Library> {
        if let Some(content) = self.content() {
            content
                .iter()
                .filter_map(|d| match d {
                    ContentDirectory::Media(lib) => match lib.library_type {
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        LibraryType::Unknown => None,
                        _ => Some(Library::new(self.client.clone(), *lib.clone())),
                    },
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Allows retrieving media, playlists, collections and other items using
    /// their rating key.
    pub async fn item_by_id(&self, rating_key: u32) -> Result<Item> {
        let path = format!("/library/metadata/{rating_key}");

        match metadata_items(&self.client, &path).await {
            Ok(items) => items.into_iter().next().ok_or(Error::ItemNotFound),
            Err(Error::UnexpectedApiResponse {
                status_code,
                content,
            }) => {
                // A 404 error indicates the item does not exist.
                if status_code == 404 {
                    Err(Error::ItemNotFound)
                } else {
                    Err(Error::UnexpectedApiResponse {
                        status_code,
                        content,
                    })
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn refresh(self) -> Result<Self> {
        Self::build(self.client, self.myplex_api_url).await
    }

    pub fn myplex(&self) -> Result<MyPlex> {
        self.myplex_with_api_url(self.myplex_api_url.clone())
    }

    pub async fn claim(self, claim_token: &str) -> Result<Self> {
        let url = format!(
            "{}?{}",
            SERVER_MYPLEX_CLAIM,
            serde_urlencoded::to_string([("token", claim_token)])?
        );
        let mut response = self.client.post(url).send().await?;

        if response.status() == StatusCode::OK {
            response.consume().await?;
            self.refresh().await
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }

    pub async fn unclaim(self) -> Result<Self> {
        let mut response = self.client.delete(SERVER_MYPLEX_ACCOUNT).send().await?;

        if response.status() == StatusCode::OK {
            response.consume().await?;
            self.refresh().await
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }

    pub fn myplex_with_api_url<U>(&self, api_url: U) -> Result<MyPlex>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        MyPlex::new(
            HttpClientBuilder::from(self.client.clone())
                .set_api_url(api_url)
                .build()?,
        )
    }

    pub fn client(&self) -> &HttpClient {
        &self.client
    }

    pub async fn preferences(&self) -> Result<Preferences<'_>> {
        Preferences::new(&self.client).await
    }
}
