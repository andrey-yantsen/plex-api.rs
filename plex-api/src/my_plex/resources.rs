use crate::media_container::{Device, DevicesMediaContainer};
use crate::{
    my_plex::{HasMyPlexToken, MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi,
};
use reqwest::StatusCode;

const RESOURCES_URL: &str = "api/resources";

impl MyPlexAccount {
    /// Returns the list of resources, registered with current MyPlex account.
    ///
    /// Resource is any available device, despite of its `provides` setting.
    pub async fn get_resources(&self) -> crate::Result<Vec<Device>> {
        let response = self.get(RESOURCES_URL).await?;
        if response.status() == StatusCode::OK {
            let mc: DevicesMediaContainer =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            let mut devices: Vec<Device> = mc.get_devices().to_owned();
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
