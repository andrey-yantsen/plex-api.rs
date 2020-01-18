use crate::media_container::{User, UsersMediaContainer};
use crate::{
    my_plex::{MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi,
};
use quick_xml;
use reqwest::StatusCode;

const USERS_URL: &str = "api/users/";

impl MyPlexAccount {
    /// Returns a list of users, who has access to the current server, except the owner.
    pub async fn get_users(&self) -> crate::Result<Vec<User>> {
        let response = self.get(USERS_URL).await?;
        if response.status() == StatusCode::OK {
            let mc: UsersMediaContainer = quick_xml::de::from_str(response.text().await?.as_str())?;
            Ok(mc.get_users().to_owned())
        } else {
            let err: MyPlexApiErrorResponse =
                quick_xml::de::from_str(response.text().await?.as_str())?;
            Err(core::convert::From::from(err))
        }
    }
}
