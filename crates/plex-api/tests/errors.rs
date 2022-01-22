mod fixtures;

use fixtures::offline::{myplex::myplex, Mocked};
use httpmock::Method::GET;
use plex_api::{url::MYPLEX_USER_INFO_PATH, MyPlex};

#[plex_api_test_helper::async_offline_test]
async fn json_error_during_ok_response(#[future] myplex: Mocked<MyPlex>) {
    let (myplex, mock_server) = myplex.await.split();
    let _mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_USER_INFO_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body("foo");
    });
    let r = dbg!(myplex.refresh().await);
    assert!(matches!(
        r.err().unwrap(),
        plex_api::Error::JsonDeserealiseError { .. }
    ));
}

#[plex_api_test_helper::async_offline_test]
async fn json_error_during_err_response(#[future] myplex: Mocked<MyPlex>) {
    let (myplex, mock_server) = myplex.await.split();
    let _mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_USER_INFO_PATH);
        then.status(400)
            .header("content-type", "text/json")
            .body("foo");
    });
    let r = dbg!(myplex.refresh().await);
    assert!(matches!(
        r.err().unwrap(),
        plex_api::Error::UnexpectedApiResponse { .. }
    ));
}

#[plex_api_test_helper::async_offline_test]
async fn correct_api_error(#[future] myplex: Mocked<MyPlex>) {
    let (myplex, mock_server) = myplex.await.split();
    let _mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_USER_INFO_PATH);
        then.status(400)
            .header("content-type", "text/json")
            .body(r#"{"errors": [{"code": 1111, "message": "test", "status": 400}]}"#);
    });
    let r = dbg!(myplex.refresh().await);
    assert!(matches!(
        r.err().unwrap(),
        plex_api::Error::MyPlexErrorResponse { .. }
    ));
}
