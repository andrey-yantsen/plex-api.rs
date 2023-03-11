use super::{client::client_authenticated, Mocked};
use plex_api::{url::MYPLEX_USER_INFO_PATH, HttpClient, MyPlex, MyPlexBuilder};
use rstest::fixture;

#[fixture]
pub async fn myplex(client_authenticated: Mocked<HttpClient>) -> Mocked<MyPlex> {
    let (client_authenticated, mock_server) = client_authenticated.split();
    let mut mock = mock_server.mock(|when, then| {
        when.path(MYPLEX_USER_INFO_PATH);
        then.status(200)
            .body_from_file("tests/mocks/myplex/api/v2/user/user_info_free.json");
    });

    let ret = MyPlexBuilder::default()
        .set_client(client_authenticated)
        .build()
        .await
        .expect("failed to login");

    mock.assert();
    mock.delete();

    Mocked::new(ret, mock_server)
}

#[fixture]
pub async fn myplex_plexpass(client_authenticated: Mocked<HttpClient>) -> Mocked<MyPlex> {
    let (client_authenticated, mock_server) = client_authenticated.split();
    let mut mock = mock_server.mock(|when, then| {
        when.path(MYPLEX_USER_INFO_PATH);
        then.status(200)
            .body_from_file("tests/mocks/myplex/api/v2/user/user_info_plexpass.json");
    });

    let ret = MyPlexBuilder::default()
        .set_client(client_authenticated)
        .build()
        .await
        .expect("failed to login");

    mock.assert();
    mock.delete();

    Mocked::new(ret, mock_server)
}
