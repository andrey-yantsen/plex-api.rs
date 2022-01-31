use super::{
    client::{client_anonymous, client_authenticated},
    Mocked,
};
use httpmock::Method::GET;
use plex_api::{url::SERVER_MEDIA_PROVIDERS, HttpClient, Server};
use rstest::fixture;

#[fixture]
pub async fn server_anonymous(client_anonymous: Mocked<HttpClient>) -> Mocked<Server> {
    let (client_anonymous, mock_server) = client_anonymous.split();

    let mut m = mock_server.mock(|when, then| {
        when.method(GET).path(SERVER_MEDIA_PROVIDERS);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/server/media/providers_free.json");
    });

    let ret = Server::new(mock_server.base_url(), client_anonymous)
        .await
        .expect("failed to get server");

    m.delete();

    Mocked::new(ret, mock_server)
}

#[fixture]
pub async fn server_authenticated(client_authenticated: Mocked<HttpClient>) -> Mocked<Server> {
    let (client_authenticated, mock_server) = client_authenticated.split();

    let mut m = mock_server.mock(|when, then| {
        when.method(GET).path(SERVER_MEDIA_PROVIDERS);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/server/media/providers_unclaimed.json");
    });

    let ret = Server::new(mock_server.base_url(), client_authenticated)
        .await
        .expect("failed to get server");

    m.delete();

    Mocked::new(ret, mock_server)
}
