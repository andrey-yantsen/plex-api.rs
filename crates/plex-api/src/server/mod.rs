use crate::{client::Client, myplex::MyPlex, ClientBuilder, Result};
use http::Uri;
use std::sync::Arc;

pub struct Server {
    #[allow(dead_code)]
    client: Arc<Client>,
    myplex_api_url: Uri,
}

impl Server {
    pub fn new<U>(url: U, client: Client) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Ok(Self {
            myplex_api_url: client.api_url.clone(),
            client: Arc::new(ClientBuilder::from(client).set_api_url(url).build()?),
        })
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
            ClientBuilder::from(self.client.as_ref().to_owned())
                .set_api_url(api_url)
                .build()?,
        )
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
