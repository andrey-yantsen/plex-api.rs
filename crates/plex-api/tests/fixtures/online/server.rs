use super::client::{client_anonymous, client_authenticated, platform};
use plex_api::{HttpClient, Server};
use rstest::fixture;
use std::time::Duration;
use tokio::time::sleep;

async fn get_server(client: HttpClient) -> Server {
    let url = std::env::var("PLEX_SERVER_URL").unwrap_or_else(|_| "".to_owned());
    if url.is_empty() {
        panic!("PLEX_SERVER_URL must be set!");
    }

    let mut attempt = 0;
    loop {
        attempt += 1;

        let r = Server::new(url.clone(), client.clone()).await;

        if let Ok(srv) = r {
            return srv;
        }

        if attempt < 10 {
            sleep(Duration::from_secs(5)).await;
        } else {
            r.expect("failed to get server");
        }
    }
}

#[fixture]
pub async fn server_unclaimed(platform: String) -> Server {
    get_server(client_anonymous::partial_1(platform)).await
}

#[fixture]
pub async fn server_claimed(platform: String) -> Server {
    get_server(client_authenticated::partial_1(platform)).await
}

#[cfg(feature = "tests_only_online_claimed_server")]
#[fixture]
pub async fn server(platform: String) -> Server {
    server_claimed::get(platform).await
}

#[cfg(not(feature = "tests_only_online_claimed_server"))]
#[fixture]
pub async fn server(platform: String) -> Server {
    server_unclaimed::get(platform).await
}
