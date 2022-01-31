use crate::{
    media_container::player::ResourcesMediaContainer, url::CLIENT_RESOURCES, HttpClient,
    HttpClientBuilder, MyPlex, Result,
};
use http::{StatusCode, Uri};
use isahc::AsyncReadResponseExt;
use std::{convert::TryFrom, sync::Arc};

#[derive(Debug, Clone)]
pub struct Player {
    #[allow(dead_code)]
    client: Arc<HttpClient>,
    myplex_api_url: Uri,
    pub media_container: ResourcesMediaContainer,
}

impl Player {
    async fn build(client: Arc<HttpClient>, myplex_api_url: Uri) -> Result<Self> {
        let mut response = client
            .get(CLIENT_RESOURCES)
            .header("Accept", "application/xml")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let response_text = response.text().await?;

                Ok(Self {
                    media_container: quick_xml::de::from_str(&response_text)?,
                    client,
                    myplex_api_url,
                })
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    pub async fn new<U>(url: U, client: HttpClient) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let myplex_api_url = client.api_url.clone();
        Self::build(
            Arc::new(HttpClientBuilder::from(client).set_api_url(url).build()?),
            myplex_api_url,
        )
        .await
    }

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
            HttpClientBuilder::from(self.client.as_ref().to_owned())
                .set_api_url(api_url)
                .build()?,
        )
    }
}
