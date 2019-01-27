use crate::media_container::{Device, MediaContainer};
use crate::my_plex::{HasMyPlexToken, MyPlexAccount, MyPlexApiErrorResponse};
use reqwest::StatusCode;
use serde_xml_rs;

impl MyPlexAccount {
    /// Returns the list of devices (players and servers), registered with current MyPlex account.
    pub fn get_devices(&self) -> crate::Result<Vec<Device>> {
        let mut response = self.get("https://plex.tv/devices.xml")?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let mut devices: Vec<Device> = mc.get_devices().unwrap_or_default();
            devices
                .iter_mut()
                .for_each(|d| d.set_auth_token(&self.auth_token));
            Ok(devices)
        } else {
            let err: MyPlexApiErrorResponse = serde_xml_rs::from_str(response.text()?.as_str())?;
            Err(crate::error::PlexApiError::from(err))
        }
    }
}
