use crate::media_container::{MediaContainer, User};
use crate::my_plex::{MyPlexAccount, MyPlexApiErrorResponse, MyPlexError, Result};
use reqwest::StatusCode;
use serde_xml_rs;

const USERS_URL: &str = "https://plex.tv/api/users/";

impl MyPlexAccount {
    pub fn get_users(&self) -> Result<Vec<User>> {
        let mut response = self.get(USERS_URL)?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let resources = mc.get_users();
            if resources.is_some() {
                Ok(resources.unwrap())
            } else {
                let ret: Vec<User> = Vec::new();
                Ok(ret)
            }
        } else {
            let err: MyPlexApiErrorResponse = serde_xml_rs::from_str(response.text()?.as_str())?;
            Err(MyPlexError::from(err))
        }
    }
}
