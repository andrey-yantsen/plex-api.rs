use crate::media_container::{Device, DevicesMediaContainer};
use crate::{
    my_plex::{HasMyPlexToken, MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi,
};
use reqwest::StatusCode;

const DEVICES_URL: &str = "devices.xml";

impl MyPlexAccount {
    /// Returns the list of devices (players and servers), registered with current MyPlex account.
    pub async fn get_devices(&self) -> crate::Result<Vec<Device>> {
        let response = self.get(DEVICES_URL).await?;
        if response.status() == StatusCode::OK {
            let mc: DevicesMediaContainer =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            let mut devices = mc.get_devices().to_owned();
            devices
                .iter_mut()
                .for_each(|d| d.set_auth_token(&self.auth_token));
            Ok(devices)
        } else {
            let err: MyPlexApiErrorResponse =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            Err(core::convert::From::from(err))
        }
    }
}
