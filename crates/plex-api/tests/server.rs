mod fixtures;

mod offline {
    use super::fixtures::offline::client::*;
    use super::fixtures::offline::server::*;
    use super::fixtures::offline::Mocked;
    use httpmock::Method::GET;
    use plex_api::{
        url::{MYPLEX_USER_INFO_PATH, SERVER_MEDIA_PROVIDERS},
        HttpClient, Server,
    };

    #[plex_api_test_helper::async_offline_test]
    #[case::free("tests/files/server/media/providers_free.json")]
    #[case::plexpass("tests/files/server/media/providers_plexpass.json")]
    #[case::unclaimed("tests/files/server/media/providers_unclaimed.json")]
    async fn load_server(client_authenticated: Mocked<HttpClient>, #[case] mock_file: &str) {
        let (client_authenticated, mock_server) = client_authenticated.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(SERVER_MEDIA_PROVIDERS)
                .header("X-Plex-Token", "fixture_auth_token");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file(mock_file);
        });

        let server_result = Server::new(mock_server.base_url(), client_authenticated).await;

        m.assert();
        server_result.expect("can't recover myplex from server");
    }

    #[plex_api_test_helper::async_offline_test]
    async fn myplex_recover_from_server(#[future] server_authenticated: Mocked<Server>) {
        let server = server_authenticated.await;
        let (server, mock_server) = server.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_USER_INFO_PATH)
                .header("X-Plex-Token", "fixture_auth_token");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/files/myplex/api/v2/user/user_info_free_guest.json");
        });

        let plex_result = server
            .myplex()
            .expect("failed to get MyPlex from the server")
            .refresh()
            .await;

        m.assert();
        plex_result.expect("can't recover myplex from server");
    }
}

mod online {
    use super::fixtures::online::client::*;
    use super::fixtures::online::server::*;
    use isahc::{config::Configurable, HttpClient as IsahcHttpClient};
    use plex_api::{HttpClient, HttpClientBuilder, Server};

    #[plex_api_test_helper::online_test_unclaimed_server]
    async fn load_server_unclaimed(#[future] server_unclaimed: Server) {
        let _ = server_unclaimed.await;
    }

    #[plex_api_test_helper::online_test_claimed_server]
    async fn load_server_claimed(#[future] server_claimed: Server) {
        let _ = server_claimed.await;
    }

    // Claim/unclaim could take long time due to unknown reasons.
    async fn get_server_with_longer_timeout(server: Server, client: HttpClient) -> Server {
        let server_url = server.client().api_url.clone();

        let client = HttpClientBuilder::from(client)
            .set_http_client(
                IsahcHttpClient::builder()
                    .timeout(std::time::Duration::from_secs(60))
                    .connect_timeout(std::time::Duration::from_secs(30))
                    .redirect_policy(isahc::config::RedirectPolicy::None)
                    .expect_continue(isahc::config::ExpectContinue::disabled())
                    .build()
                    .expect("failed to create testing http client"),
            )
            .build()
            .expect("failed to build client");

        Server::new(server_url, client)
            .await
            .expect("failed to get server")
    }

    #[allow(unused_attributes)]
    #[plex_api_test_helper::online_test_claimed_server]
    #[ignore = "Must be run manually"]
    async fn claim_server(#[future] server_unclaimed: Server, client_authenticated: HttpClient) {
        let server =
            get_server_with_longer_timeout(server_unclaimed.await, client_authenticated).await;

        let myplex = server
            .myplex()
            .expect("failed to get MyPlex from the server");
        let claim_token = myplex
            .claim_token()
            .await
            .expect("failed to get claim token")
            .to_string();

        server
            .claim(&claim_token)
            .await
            .expect("failed to claim server")
            .unclaim()
            .await
            .expect("failed to unclaim server");
    }

    #[allow(unused_attributes)]
    #[plex_api_test_helper::online_test_claimed_server]
    #[ignore = "Must be run manually"]
    async fn unclaim_server(#[future] server_claimed: Server) {
        let server = server_claimed.await;
        let client = server.client().to_owned();

        let server = get_server_with_longer_timeout(server, client).await;

        server.unclaim().await.expect("failed to unclaim server");
    }
}
