use crate::base_headers;
use crate::my_plex::{MyPlex, MyPlexApiErrorResponse, MyPlexError, Result};
use reqwest::{header::ACCEPT, Client, StatusCode};

const MYPLEX_LOGIN_URL: &str = "https://plex.tv/api/v2/users/signin";
const MYPLEX_ACCOUNT_INFO_URL: &str = "https://plex.tv/api/v2/user?includeSubscriptions=1";

impl MyPlex {
    pub fn login(username: &str, password: &str) -> Result<Self> {
        let params = [
            ("login", username),
            ("password", password),
            ("mememberMe", "true"),
        ];

        let mut response = Client::new()
            .post(MYPLEX_LOGIN_URL)
            .form(&params)
            .headers(base_headers())
            .header(ACCEPT, "application/json")
            .send()?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<MyPlex>()?),
            _ => Err(MyPlexError::from(
                response.json::<MyPlexApiErrorResponse>()?,
            )),
        }
    }

    pub fn by_token(token: &str) -> Result<Self> {
        let mut response = Client::new()
            .get(MYPLEX_ACCOUNT_INFO_URL)
            .headers(base_headers())
            .header(ACCEPT, "application/json")
            .header("X-Plex-Token", token)
            .send()?;

        match response.status() {
            StatusCode::CREATED => Ok(response.json::<MyPlex>()?),
            _ => Err(MyPlexError::from(
                response.json::<MyPlexApiErrorResponse>()?,
            )),
        }
    }
}
