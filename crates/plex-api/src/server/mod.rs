use crate::{
    client::Client,
    media_container::{server::Server as ServerMediaContainer, MediaContainerWrapper},
    myplex::MyPlex,
    url::{SERVER_MEDIA_PROVIDERS, SERVER_MYPLEX_ACCOUNT, SERVER_MYPLEX_CLAIM},
    ClientBuilder, Result,
};
use core::convert::TryFrom;
use http::{StatusCode, Uri};
use isahc::AsyncReadResponseExt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Server {
    #[allow(dead_code)]
    client: Arc<Client>,
    myplex_api_url: Uri,
    #[allow(dead_code)]
    media_container: Option<ServerMediaContainer>,
}

impl Server {
    pub async fn new<U>(url: U, client: Client) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let ret = Self {
            myplex_api_url: client.api_url.clone(),
            client: Arc::new(ClientBuilder::from(client).set_api_url(url).build()?),
            media_container: None,
        };

        ret.refresh().await
    }

    pub async fn refresh(self) -> Result<Self> {
        let mut response = self
            .client
            .get(SERVER_MEDIA_PROVIDERS)
            .header("Accept", "application/json")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let media_container_wrapper: MediaContainerWrapper<ServerMediaContainer> =
                    response.json().await?;

                Ok(Self {
                    media_container: Some(media_container_wrapper.media_container),
                    ..self
                })
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    pub fn myplex(&self) -> Result<MyPlex> {
        self.myplex_with_api_url(self.myplex_api_url.clone())
    }

    pub async fn claim(self, claim_token: &str) -> Result<Self> {
        let url = format!(
            "{}?{}",
            SERVER_MYPLEX_CLAIM,
            serde_urlencoded::to_string(&[("token", claim_token)])?
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
            ClientBuilder::from(self.client.as_ref().to_owned())
                .set_api_url(api_url)
                .build()?,
        )
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
