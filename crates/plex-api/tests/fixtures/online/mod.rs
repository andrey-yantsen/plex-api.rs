pub mod client;
pub mod server;

use isahc::{config::Configurable, HttpClient};
use plex_api::HttpClientBuilder;
use rstest::fixture;

#[fixture]
pub fn client_builder() -> HttpClientBuilder {
    let mut builder = HttpClientBuilder::default().set_http_client(
        HttpClient::builder()
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
