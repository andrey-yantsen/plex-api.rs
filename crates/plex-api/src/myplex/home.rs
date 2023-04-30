use crate::{
    media_container::home::{User, UsersResponse},
    url::{MYPLEX_USERS, MYPLEX_USER_SWITCH},
    HttpClient, MyPlex, Result,
};

use super::account::MyPlexAccount;

pub struct HomeManager {
    pub(crate) client: HttpClient,
}

#[derive(Debug, Clone)]
pub enum SwitchableUser {
    UserUuid(String),
    UserId(u64),
}

impl SwitchableUser {
    pub fn id(&self) -> String {
        match self {
            SwitchableUser::UserUuid(uuid) => uuid.to_owned(),
            SwitchableUser::UserId(id) => id.to_string(),
        }
    }
}

impl From<User> for SwitchableUser {
    fn from(val: User) -> Self {
        SwitchableUser::UserUuid(val.uuid)
    }
}

impl From<&User> for SwitchableUser {
    fn from(val: &User) -> Self {
        SwitchableUser::UserUuid(val.uuid.clone())
    }
}

impl From<String> for SwitchableUser {
    fn from(val: String) -> Self {
        SwitchableUser::UserUuid(val)
    }
}

impl From<&str> for SwitchableUser {
    fn from(val: &str) -> Self {
        SwitchableUser::UserUuid(val.to_owned())
    }
}

impl From<u64> for SwitchableUser {
    fn from(val: u64) -> Self {
        SwitchableUser::UserId(val)
    }
}

impl HomeManager {
    pub async fn users(&self) -> Result<Vec<User>> {
        Ok(self
            .client
            .get(MYPLEX_USERS)
            .xml::<UsersResponse>()
            .await?
            .users)
    }

    pub async fn switch_user(
        &self,
        myplex: MyPlex,
        user: impl Into<SwitchableUser>,
        pin: Option<&str>,
    ) -> Result<MyPlex> {
        let mut myplex = myplex;
        let account: MyPlexAccount = self
            .client
            .post(format!(
                "{}?pin={}",
                MYPLEX_USER_SWITCH.replace("{uuid}", &user.into().id()),
                pin.unwrap_or_default()
            ))
            .json()
            .await?;
        myplex.client = myplex.client.set_x_plex_token(account.auth_token.clone());
        myplex.account = Some(account);

        Ok(myplex)
    }
}
