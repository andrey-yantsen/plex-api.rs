use crate::{
    base_headers, get_http_client,
    my_plex::{MyPlexAccount, MyPlexApiErrorResponse},
};

const MYPLEX_LOGIN_URL: &str = "https://plex.tv/api/v2/users/signin";
const MYPLEX_ACCOUNT_INFO_URL: &str = "https://plex.tv/api/v2/user?includeSubscriptions=1";

impl MyPlexAccount {
    /// Log in to [MyPlex](http://app.plex.tv) using username and password.
    pub async fn login(username: &str, password: &str) -> crate::Result<Self> {
        let params = [
            ("login", username),
            ("password", password),
            ("mememberMe", "true"),
        ];

        let response = get_http_client()?
            .post(MYPLEX_LOGIN_URL)
            .form(&params)
            .headers(base_headers())
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        MyPlexAccount::handle_login(response).await
    }

    /// Log in to [MyPlex](http://app.plex.tv) using existing authentication token.
    pub async fn by_token(auth_token: &str) -> crate::Result<Self> {
        let response = get_http_client()?
            .get(MYPLEX_ACCOUNT_INFO_URL)
            .headers(base_headers())
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Plex-Token", auth_token)
            .send()
            .await?;

        MyPlexAccount::handle_login(response).await
    }

    async fn handle_login(r: reqwest::Response) -> crate::Result<Self> {
        match r.status() {
            reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {
                Ok(r.json::<MyPlexAccount>().await?)
            }
            _ => Err(crate::error::PlexApiError::from(
                r.json::<MyPlexApiErrorResponse>().await?,
            )),
        }
    }
}
