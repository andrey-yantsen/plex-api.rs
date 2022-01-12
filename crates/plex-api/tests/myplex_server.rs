use plex_api::url::MYPLEX_USER_INFO_PATH;

#[plex_api_test_helper::async_offline_test]
async fn myplex_recover_from_server(myplex: MyPlex) {
    let srv = myplex.srv().await.expect("can't get server");

    let m = mockito::mock("GET", MYPLEX_USER_INFO_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free_guest.json")
        .match_header("X-Plex-Token", "auth_token")
        .create();

    let plex_result = srv
        .myplex()
        .expect("failed to get MyPlex from the server")
        .refresh()
        .await;

    m.assert();
    plex_result.expect("can't recover myplex from server");
}

#[plex_api_test_helper::async_offline_test]
async fn myplex_recover_from_server2(srv: Server) {
    let m = mockito::mock("GET", MYPLEX_USER_INFO_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free_guest.json")
        .match_header(
            "X-Plex-Token",
            "auth_token_offline_myplex_recover_from_server2",
        )
        .create();

    let plex_result = srv
        .myplex()
        .expect("failed to get MyPlex from the server")
        .refresh()
        .await;

    m.assert();
    plex_result.expect("can't recover myplex from server");
}
