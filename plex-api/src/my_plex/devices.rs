use crate::media_container::{Device, MediaContainer};
use crate::{
    my_plex::{HasMyPlexToken, MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi,
};
use reqwest::StatusCode;
use serde_xml_rs;

const DEVICES_URL: &str = "devices.xml";

impl MyPlexAccount {
    /// Returns the list of devices (players and servers), registered with current MyPlex account.
    pub async fn get_devices(&self) -> crate::Result<Vec<Device>> {
        let response = self.get(DEVICES_URL).await?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text().await?.as_str())?;
            let mut devices: Vec<Device> = mc.get_devices().unwrap_or_default();
            devices
                .iter_mut()
                .for_each(|d| d.set_auth_token(&self.auth_token));
            Ok(devices)
        } else {
            let err: MyPlexApiErrorResponse =
                serde_xml_rs::from_str(response.text().await?.as_str())?;
            Err(crate::error::PlexApiError::from(err))
        }
    }
}
