use crate::media_container::{Device, MediaContainer};
use crate::my_plex::{MyPlexAccount, MyPlexError, Result};
use reqwest::StatusCode;
use serde_xml_rs;

impl MyPlexAccount {
    pub fn get_devices(&self) -> Result<Vec<Device>> {
        let mut response = self.get("https://plex.tv/devices.xml")?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let devices = mc.get_devices();
            if devices.is_some() {
                Ok(devices.unwrap())
            } else {
                let ret: Vec<Device> = Vec::new();
                Ok(ret)
            }
        } else {
            Err(MyPlexError {})
        }
    }
}
