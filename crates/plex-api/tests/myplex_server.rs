mod fixtures;

use crate::fixtures::offline::Mocked;
use fixtures::offline::myplex::*;
use fixtures::offline::server::*;
use httpmock::Method::GET;
use plex_api::{url::MYPLEX_USER_INFO_PATH, MyPlex, Server};

#[plex_api_test_helper::async_offline_test]
async fn myplex_recover_from_server(#[future] myplex: Mocked<MyPlex>) {
    let myplex = myplex.await;
    let (myplex, mock_server) = myplex.decompose();

    let srv = myplex.srv().await.expect("can't get server");

    let m = mock_server.mock(|when, then| {
        when.method(GET)
            .path(MYPLEX_USER_INFO_PATH)
            .header("X-Plex-Token", "auth_token");
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/user_info_free_guest.json");
    });

    let plex_result = srv
        .myplex()
        .expect("failed to get MyPlex from the server")
        .refresh()
        .await;

    m.assert();
    plex_result.expect("can't recover myplex from server");
}

#[plex_api_test_helper::async_offline_test]
async fn myplex_recover_from_server2(#[future] server_authenticated: Mocked<Server>) {
    let server = server_authenticated.await;
    let (server, mock_server) = server.decompose();

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
