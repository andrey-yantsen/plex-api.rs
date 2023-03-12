use crate::{
    http_client::HttpClient,
    media_container::devices::{DevicesMediaContainer, Feature},
    url::{MYPLEX_DEVICES, MYPLEX_RESOURCES},
    Error, Player, Result, Server,
};
use futures::{future::select_ok, FutureExt};
use secrecy::ExposeSecret;

pub struct DeviceManager {
    pub client: HttpClient,
}

impl DeviceManager {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    async fn get_devices_internal<'a, 'b>(&'a self, url: &'b str) -> Result<Vec<Device<'a>>> {
        let container: DevicesMediaContainer = self
            .client
            .get(url)
            .header("Accept", "application/xml")
            .xml()
            .await?;

        Ok(container
            .devices
            .into_iter()
            .map(|device| Device {
                inner: device,
                client: &self.client,
            })
            .collect())
    }

    pub async fn get_devices(&self) -> Result<Vec<Device<'_>>> {
        self.get_devices_internal(MYPLEX_DEVICES).await
    }

    pub async fn get_resources(&self) -> Result<Vec<Device<'_>>> {
        self.get_devices_internal(MYPLEX_RESOURCES).await
    }
}

#[derive(Debug, Clone)]
pub struct Device<'a> {
    inner: crate::media_container::devices::Device,
    client: &'a HttpClient,
}

impl Device<'_> {
    pub fn provides(&self, feature: Feature) -> bool {
        self.inner.provides.contains(&feature)
    }

    pub fn identifier(&self) -> &str {
        &self.inner.client_identifier
    }

    pub fn access_token(&self) -> Option<&str> {
        self.inner
            .access_token
            .as_ref()
            .map(|v| v.expose_secret().as_str())
    }

    pub async fn connect(&self) -> Result<DeviceConnection> {
        if !self.inner.provides.contains(&Feature::Server)
            && !self.inner.provides.contains(&Feature::Player)
        {
            return Err(Error::DeviceConnectionNotSupported);
        }

        if let Some(access_token) = self.inner.access_token.as_ref() {
            if access_token.expose_secret() != self.client.x_plex_token() {
                todo!("connection to a shared device is not implemented");
            }
        }

        if !self.inner.connections.is_empty() {
            if self.inner.provides.contains(&Feature::Server) {
                let futures = self
                    .inner
                    .connections
                    .iter()
                    .map(|connection| {
                        crate::Server::new(&connection.uri, self.client.to_owned()).boxed()
                    })
                    .collect::<Vec<_>>();

                let (server, _) = select_ok(futures).await?;
                Ok(DeviceConnection::Server(Box::new(server)))
            } else {
                let futures = self
                    .inner
                    .connections
                    .iter()
                    .map(|connection| {
                        crate::Player::new(&connection.uri, self.client.to_owned()).boxed()
                    })
                    .collect::<Vec<_>>();

                let (player, _) = select_ok(futures).await?;
                Ok(DeviceConnection::Player(Box::new(player)))
            }
        } else {
            Err(Error::DeviceConnectionsIsEmpty)
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeviceConnection {
    Server(Box<Server>),
    Player(Box<Player>),
}
