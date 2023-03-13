use crate::{
    media_container::player::ResourcesMediaContainer, url::CLIENT_RESOURCES, HttpClient,
    HttpClientBuilder, MyPlex, Result,
};
use http::Uri;
use std::{convert::TryFrom, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Player {
    #[allow(dead_code)]
    client: HttpClient,
    myplex_api_url: Uri,
    pub media_container: ResourcesMediaContainer,
}

impl Player {
    async fn build(client: HttpClient, myplex_api_url: Uri) -> Result<Self> {
        Ok(Self {
            media_container: client
                .get(CLIENT_RESOURCES)
                .header("Accept", "application/xml")
                .xml()
                .await?,
            client,
            myplex_api_url,
        })
    }

    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new<U>(url: U, client: HttpClient) -> Result<Self>
    where
        U: Debug,
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

    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn refresh(self) -> Result<Self> {
        Self::build(self.client, self.myplex_api_url).await
    }

    pub fn myplex(&self) -> Result<MyPlex> {
        self.myplex_with_api_url(self.myplex_api_url.clone())
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
}
