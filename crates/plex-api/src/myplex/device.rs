use std::sync::Arc;

use crate::{
    client::Client,
    media_container::devices::DevicesMediaContainer,
    url::{MYPLEX_DEVICES, MYPLEX_RESOURCES},
    Result,
};
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
    #[allow(dead_code)]
    inner: crate::media_container::devices::Device,
    client: &'a Client,
}

impl Device<'_> {
    pub async fn connect(&self) -> Result<crate::Server> {
        if let Some(connections) = &self.inner.connections {
            crate::Server::new(&connections[0].uri, self.client.to_owned())
        } else {
            Err(crate::Error::UselessOtp)
        }
    }
}
