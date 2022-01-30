use std::sync::Arc;

use crate::{
    client::Client,
    media_container::devices::{DevicesMediaContainer, Feature},
    url::{MYPLEX_DEVICES, MYPLEX_RESOURCES},
    Error, Result,
};
use futures::{future::select_ok, FutureExt};
use http::StatusCode;
use isahc::AsyncReadResponseExt;

pub struct DeviceManager {
    pub client: Arc<Client>,
}

impl DeviceManager {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_devices_internal<'a, 'b>(&'a self, url: &'b str) -> Result<Vec<Device<'a>>> {
        let mut response = self
            .client
            .get(url)
            .header("Accept", "application/xml")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let response_text = response.text().await?;

                let container: DevicesMediaContainer = quick_xml::de::from_str(&response_text)?;

                Ok(container
                    .devices
                    .into_iter()
                    .map(|device| Device {
                        inner: device,
                        client: &self.client,
                    })
                    .collect())
            }
            _ => Err(crate::Error::from_response(response).await),
        }
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
    client: &'a Client,
}

impl Device<'_> {
    pub async fn connect(&self) -> Result<DeviceConnection> {
        if !self.inner.provides.contains(&Feature::Server) {
            return Err(Error::DeviceConnectionNotSupported);
        }

        if let Some(connections) = &self.inner.connections {
            let futures = connections
                .iter()
                .map(|connection| {
                    crate::Server::new(&connection.uri, self.client.to_owned()).boxed()
                })
                .collect::<Vec<_>>();

            let (server, _) = select_ok(futures).await?;
            Ok(DeviceConnection::Server(server))
        } else {
            Err(Error::DeviceConnectionsIsEmpty)
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeviceConnection {
    Server(crate::Server),
}
