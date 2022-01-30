use super::client::{client_anonymous, client_authenticated};
use plex_api::{Client, Server};
use rstest::fixture;

async fn get_server(client: Client) -> Server {
    let url = std::env::var("PLEX_SERVER_URL").unwrap_or_else(|_| "".to_owned());
    if url.is_empty() {
        panic!("PLEX_SERVER_URL must be set!");
    }

    Server::new(url, client)
        .await
        .expect("failed to get server")
}

#[fixture]
pub async fn server_unclaimed(client_anonymous: Client) -> Server {
    get_server(client_anonymous).await
}

#[fixture]
pub async fn server_claimed(client_authenticated: Client) -> Server {
    get_server(client_authenticated).await
}
