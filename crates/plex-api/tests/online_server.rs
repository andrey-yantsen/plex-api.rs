mod fixtures;

use fixtures::online::server::*;
use plex_api::Server;

#[plex_api_test_helper::online_anonymous_test]
async fn test_connection_anonymous(#[future] server_anonymous: Server) {
    let _server_result = server_anonymous.await;
}

#[plex_api_test_helper::online_authenticated_test]
async fn test_connection_authenticated(#[future] server_authenticated: Server) {
    let _server_result = server_authenticated.await;
}

#[allow(unused_attributes)]
#[plex_api_test_helper::online_authenticated_test]
#[ignore = "Must be run manually"]
async fn unclaim_server(#[future] server_authenticated: Server) {
    let server = server_authenticated.await;
    server.unclaim().await.unwrap();
}
