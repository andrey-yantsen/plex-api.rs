use super::{
    client::{client_anonymous, client_authenticated},
    Mocked,
};
use plex_api::{Client, Server};
use rstest::fixture;

#[fixture]
pub async fn server_anonymous(client_anonymous: Mocked<Client>) -> Mocked<Server> {
    let (client_anonymous, mock_server) = client_anonymous.split();

    let ret = Server::new(mock_server.base_url(), client_anonymous)
        .await
        .expect("failed to get server");

    Mocked::new(ret, mock_server)
}

#[fixture]
pub async fn server_authenticated(client_authenticated: Mocked<Client>) -> Mocked<Server> {
    let (client_authenticated, mock_server) = client_authenticated.split();

    let ret = Server::new(mock_server.base_url(), client_authenticated)
        .await
        .expect("failed to get server");

    Mocked::new(ret, mock_server)
}
