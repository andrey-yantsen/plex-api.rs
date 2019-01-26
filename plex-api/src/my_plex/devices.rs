use crate::media_container::{Device, MediaContainer};
use crate::my_plex::{MyPlexAccount, MyPlexApiErrorResponse, MyPlexError, Result};
use reqwest::StatusCode;
use serde_xml_rs;

impl MyPlexAccount {
    /// Returns the list of devices (players and servers), registered with current MyPlex account.
    pub fn get_devices(&self) -> Result<Vec<Device>> {
        let mut response = self.get("https://plex.tv/devices.xml")?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let devices = mc.get_devices();
            Ok(devices.unwrap_or_default())
        } else {
            let err: MyPlexApiErrorResponse = serde_xml_rs::from_str(response.text()?.as_str())?;
            Err(MyPlexError::from(err))
        }
    }
}
