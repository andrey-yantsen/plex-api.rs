use crate::base_headers;
use crate::get_http_client;
use crate::my_plex::{MyPlexAccount, MyPlexApiErrorResponse, MyPlexError, Result};

const MYPLEX_LOGIN_URL: &str = "https://plex.tv/api/v2/users/signin";
const MYPLEX_ACCOUNT_INFO_URL: &str = "https://plex.tv/api/v2/user?includeSubscriptions=1";

impl MyPlexAccount {
    pub fn login(username: &str, password: &str) -> Result<Self> {
        let params = [
            ("login", username),
            ("password", password),
            ("mememberMe", "true"),
        ];

        let mut response = get_http_client()?
            .post(MYPLEX_LOGIN_URL)
            .form(&params)
            .headers(base_headers())
            .header(reqwest::header::ACCEPT, "application/json")
            .send()?;
        MyPlexAccount::handle_login(&mut response)
    }

    pub fn by_token(auth_token: &str) -> Result<Self> {
        let mut response = get_http_client()?
            .get(MYPLEX_ACCOUNT_INFO_URL)
            .headers(base_headers())
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Plex-Token", auth_token)
            .send()?;

        MyPlexAccount::handle_login(&mut response)
    }

    fn handle_login(r: &mut reqwest::Response) -> Result<Self> {
        match r.status() {
            reqwest::StatusCode::CREATED => Ok(r.json::<MyPlexAccount>()?),
            _ => Err(MyPlexError::from(r.json::<MyPlexApiErrorResponse>()?)),
        }
    }
}
