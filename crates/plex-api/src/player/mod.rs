use crate::{
    media_container::player::ResourcesMediaContainer,
    url::{CLIENT_RESOURCES, SERVER_SYSTEM_PROXY},
    HttpClient, HttpClientBuilder, MyPlex, Result, Server,
};
use http::{uri::PathAndQuery, Uri};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Player {
    client: HttpClient,
    _media_container: ResourcesMediaContainer,
    _last_command_id: u64,
    pub myplex_api_url: Uri,
}

impl Player {
    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new<U>(url: U, client: HttpClient) -> Result<Self>
    where
        U: Debug,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let myplex_api_url = client.api_url.clone();
        Ok(Self {
            _media_container: client
                .get(CLIENT_RESOURCES)
                .header("Accept", "application/xml")
                .xml()
                .await?,
            client,
            myplex_api_url,
            _last_command_id: 0,
        })
    }

    #[tracing::instrument(level = "debug", skip(server))]
    pub async fn via_proxy<U>(url: U, server: &Server) -> Result<Self>
    where
        U: Display + Debug,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let mut client = server.client().clone();

        let path_and_query =
            PathAndQuery::try_from(CLIENT_RESOURCES).map_err(Into::<http::Error>::into)?;
        let mut uri_parts = Uri::try_from(url)
            .map_err(Into::<http::Error>::into)?
            .into_parts();
        uri_parts.path_and_query = Some(path_and_query);
        let uri = Uri::from_parts(uri_parts).map_err(Into::<http::Error>::into)?;

        let media_container: ResourcesMediaContainer = client
            .get(SERVER_SYSTEM_PROXY)
            .header("X-Plex-Url", format!("{uri}"))
            .xml()
            .await?;
        client
            .x_plex_target_client_identifier
            .clone_from(&media_container.player.machine_identifier);
        Ok(Self {
            _media_container: media_container,
            client,
            myplex_api_url: server.myplex_api_url.clone(),
            _last_command_id: 0,
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
        Ok(MyPlex::new(
            HttpClientBuilder::from(self.client.clone())
                .set_api_url(api_url)
                .build()?,
        ))
    }

    pub fn client(&self) -> &HttpClient {
        &self.client
    }
}
