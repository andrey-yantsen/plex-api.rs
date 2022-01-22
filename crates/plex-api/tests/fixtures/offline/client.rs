use super::{client_builder, Mocked};
use plex_api::{Client, ClientBuilder};
use rstest::fixture;

#[fixture]
pub fn client_anonymous(client_builder: Mocked<ClientBuilder>) -> Mocked<Client> {
    let (client_builder, mock_server) = client_builder.split();

    Mocked::new(
        client_builder
            .build()
            .expect("failed to create testing http client"),
        mock_server,
    )
}

#[fixture]
pub fn client_authenticated(client_builder: Mocked<ClientBuilder>) -> Mocked<Client> {
    let (client_builder, mock_server) = client_builder.split();

    Mocked::new(
        client_builder
            .set_x_plex_token("fixture_auth_token".to_owned())
            .build()
            .expect("failed to create testing http client"),
        mock_server,
    )
}
