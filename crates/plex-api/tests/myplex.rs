use mockito::{mock, Matcher};
use plex_api::url::{MYPLEX_CLAIM_TOKEN_PATH, MYPLEX_PRIVACY_PATH};

#[plex_api_test_helper::async_offline_test]
async fn privacy(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_PRIVACY_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/privacy.json")
        .create();

    let privacy_result = myplex.privacy().await;
    m.assert();

    let mut privacy = privacy_result.expect("failed to get privacy");

    assert!(
        privacy.opt_out_playback,
        "Unexpected opt_out_playback value"
    );
    assert!(
        privacy.opt_out_library_stats,
        "Unexpected opt_out_library_stats value"
    );

    let m = mock("PUT", MYPLEX_PRIVACY_PATH)
        .with_status(204)
        .with_header("content-type", "text/json")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("optOutPlayback".to_string(), "0".to_string()),
            Matcher::UrlEncoded("optOutLibraryStats".to_string(), "0".to_string()),
        ]))
        .create();

    let privacy_update_result = privacy.update(false, false).await;
    m.assert();

    assert!(
        privacy_update_result.is_ok(),
        "failed to update privacy settings"
    );

    assert!(
        !privacy.opt_out_playback,
        "Unexpected opt_out_playback value"
    );
    assert!(
        !privacy.opt_out_library_stats,
        "Unexpected opt_out_library_stats value"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn claim_token(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_CLAIM_TOKEN_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/claim/token.json")
        .create();

    let token = myplex
        .claim_token()
        .await
        .expect("failed to get claim_token");
    m.assert();
    drop(m);

    assert_eq!(
        token.to_string(),
        "claim-TOKEN",
        "Received unexpected token"
    );
    assert!(!token.is_expired(), "Token is expired");

    let m = mock("GET", MYPLEX_CLAIM_TOKEN_PATH)
        .with_status(503)
        .with_header("content-type", "text/json")
        .with_body(r#"{"error": "Something went wrong"}"#)
        .create();

    let token_result = myplex.claim_token().await;
    m.assert();
    drop(m);

    assert!(
        token_result.is_err(),
        "Expected error when getting claim_token"
    );

    assert!(
        matches!(
            token_result.err().unwrap(),
            plex_api::Error::FailedToGetClaimToken(_)
        ),
        "Unexpected error returned"
    );
}
