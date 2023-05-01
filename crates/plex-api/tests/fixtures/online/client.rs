use super::client_builder;
use plex_api::{HttpClient, HttpClientBuilder};
use rstest::fixture;

#[fixture]
pub fn platform() -> String {
    sys_info::os_type().unwrap_or_else(|_| "unknown".to_string())
}

#[fixture]
pub fn client_anonymous(platform: String, client_builder: HttpClientBuilder) -> HttpClient {
    client_builder
        .set_x_plex_platform(platform)
        .build()
        .expect("failed to create testing http client")
}

#[fixture]
pub fn client_authenticated(platform: String, client_builder: HttpClientBuilder) -> HttpClient {
    let token = std::env::var("PLEX_API_AUTH_TOKEN").unwrap_or_else(|_| "".to_owned());
    if token.is_empty() {
        panic!("PLEX_API_AUTH_TOKEN must be set!");
    }

    client_builder
        .set_x_plex_token(token)
        .set_x_plex_platform(platform)
        .build()
        .expect("failed to create testing http client")
}
