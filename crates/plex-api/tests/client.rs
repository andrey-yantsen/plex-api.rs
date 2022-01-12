use std::time::Duration;

use isahc::HttpClient;
use mockito::{mock, Matcher};
use plex_api::ClientBuilder;

#[plex_api_test_helper::async_offline_test]
async fn default_client() {
    let client = ClientBuilder::new(mockito::server_url())
        .build()
        .expect("failed to build default client");

    let m = mock("GET", "/")
        .with_status(200)
        .match_header("X-Plex-Provides", "controller")
        .match_header("X-Plex-Platform", Matcher::Any)
        .match_header("X-Plex-Platform-Version", Matcher::Any)
        .match_header("X-Plex-Product", env!("CARGO_PKG_NAME"))
        .match_header("X-Plex-Version", env!("CARGO_PKG_VERSION"))
        .match_header("X-Plex-Device", Matcher::Any)
        .match_header("X-Plex-Device-Name", Matcher::Any)
        .match_header("X-Plex-Sync-Version", "2")
        .match_header("X-Plex-Client-Identifier", Matcher::Any)
        .match_header("X-Plex-Token", Matcher::Missing)
        .create();

    let get_result = client.get("/").send().await;

    m.assert();

    get_result.expect("failed to perform first http request");
}

#[plex_api_test_helper::async_offline_test]
async fn customized_client() {
    let client = ClientBuilder::new(mockito::server_url())
        .set_x_plex_token("auth_token".to_owned())
        .set_x_plex_client_identifier("client_id".to_owned())
        .set_x_plex_provides(vec!["provides1".to_owned(), "provides2".to_owned()])
        .set_x_plex_platform("platform".to_owned())
        .set_x_plex_platform_version("platform_version".to_owned())
        .set_x_plex_product("product".to_owned())
        .set_x_plex_version("product_version".to_owned())
        .set_x_plex_device("device".to_owned())
        .set_x_plex_device_name("device_name".to_owned())
        .build()
        .expect("failed to build custom client");

    let m = mock("GET", "/")
        .with_status(200)
        .match_header("X-Plex-Provides", "provides1,provides2")
        .match_header("X-Plex-Platform", "platform")
        .match_header("X-Plex-Platform-Version", "platform_version")
        .match_header("X-Plex-Product", "product")
        .match_header("X-Plex-Version", "product_version")
        .match_header("X-Plex-Device", "device")
        .match_header("X-Plex-Device-Name", "device_name")
        .match_header("X-Plex-Sync-Version", "2")
        .match_header("X-Plex-Client-Identifier", "client_id")
        .match_header("X-Plex-Token", "auth_token")
        .create();

    let get_result = client.get("/").send().await;

    m.assert();

    get_result.expect("failed to perform first http request");
}

#[plex_api_test_helper::async_offline_test]
async fn auth_token_updated_after_build() {
    let client = ClientBuilder::new(mockito::server_url())
        .build()
        .expect("failed to build default client")
        .set_x_plex_token("auth_token".to_owned());

    let m = mock("GET", "/")
        .with_status(200)
        .match_header("X-Plex-Provides", "controller")
        .match_header("X-Plex-Platform", Matcher::Any)
        .match_header("X-Plex-Platform-Version", Matcher::Any)
        .match_header("X-Plex-Product", env!("CARGO_PKG_NAME"))
        .match_header("X-Plex-Version", env!("CARGO_PKG_VERSION"))
        .match_header("X-Plex-Device", Matcher::Any)
        .match_header("X-Plex-Device-Name", Matcher::Any)
        .match_header("X-Plex-Sync-Version", "2")
        .match_header("X-Plex-Client-Identifier", Matcher::Any)
        .match_header("X-Plex-Token", "auth_token")
        .create();

    let get_result = client.get("/").send().await;

    m.assert();

    get_result.expect("failed to perform first http request");
}

#[plex_api_test_helper::async_offline_test]
async fn custom_http_client() {
    use isahc::config::Configurable as _;

    let client = ClientBuilder::default()
        .set_http_client(
            HttpClient::builder()
                .timeout(Duration::from_secs(1))
                .connect_timeout(Duration::from_secs(1))
                .proxy(Some(
                    mockito::server_url()
                        .parse()
                        .expect("failed to parse proxy url"),
                ))
                .build()
                .expect("failed to create testing http client"),
        )
        .set_api_url("http://plex.tv")
        .set_x_plex_token("auth_token".to_owned())
        .set_x_plex_client_identifier("client_id".to_owned())
        .set_x_plex_provides(vec!["provides1".to_owned(), "provides2".to_owned()])
        .set_x_plex_platform("platform".to_owned())
        .set_x_plex_platform_version("platform_version".to_owned())
        .set_x_plex_product("product".to_owned())
        .set_x_plex_version("product_version".to_owned())
        .set_x_plex_device("device".to_owned())
        .set_x_plex_device_name("device_name".to_owned())
        .build()
        .expect("failed to build client");

    let m = mock("GET", "http://plex.tv/index.html")
        .with_status(200)
        .match_header("Host", "plex.tv")
        .match_header("Proxy-connection", Matcher::Any)
        .match_header("X-Plex-Provides", "provides1,provides2")
        .match_header("X-Plex-Platform", "platform")
        .match_header("X-Plex-Platform-Version", "platform_version")
        .match_header("X-Plex-Product", "product")
        .match_header("X-Plex-Version", "product_version")
        .match_header("X-Plex-Device", "device")
        .match_header("X-Plex-Device-Name", "device_name")
        .match_header("X-Plex-Sync-Version", "2")
        .match_header("X-Plex-Client-Identifier", "client_id")
        .match_header("X-Plex-Token", "auth_token")
        .create();

    let get_result = client.get("/index.html").send().await;

    m.assert();

    get_result.expect("failed to perform first http request");
}
