#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
use crate::Server;

mod headers;
mod media_container;

#[cfg(feature = "test_connect_authenticated")]
async fn get_server_authenticated() -> Server {
    use std::env;
    let srv: Result<Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        Server::connect_auth(&server_url, &auth_token).await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.ok().unwrap()
}

#[cfg(feature = "test_connect_anonymous")]
async fn get_server_anonymous() -> Server {
    use std::env;
    let srv: Result<Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        Server::connect(&server_url).await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.ok().unwrap()
}
