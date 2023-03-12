pub mod client;
pub mod server;

use client::client_authenticated;
use isahc::{config::Configurable, HttpClient as IsahcHttpClient};
use plex_api::{HttpClient as PlexHttpClient, HttpClientBuilder, MyPlex, MyPlexBuilder};
use rstest::fixture;

#[fixture]
pub fn client_builder() -> HttpClientBuilder {
    let mut builder = HttpClientBuilder::default().set_http_client(
        IsahcHttpClient::builder()
            .timeout(std::time::Duration::from_secs(10))
            .connect_timeout(std::time::Duration::from_secs(2))
            .redirect_policy(isahc::config::RedirectPolicy::None)
            .expect_continue(isahc::config::ExpectContinue::disabled())
            .build()
            .expect("failed to create testing http client"),
    );

    let client_id = std::env::var("X_PLEX_CLIENT_IDENTIFIER").unwrap_or_else(|_| "".to_owned());

    if !client_id.is_empty() {
        builder = builder.set_x_plex_client_identifier(client_id);
    }

    builder
}

#[fixture]
pub async fn myplex(client_authenticated: PlexHttpClient) -> MyPlex {
    MyPlexBuilder::default()
        .set_client(client_authenticated)
        .build()
        .await
        .unwrap()
}
