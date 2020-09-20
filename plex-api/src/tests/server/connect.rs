test_case_online_all!(_decode_server_online);

// If methods get_server_authenticated() / get_server_anonymous() will not fail — then everything's
// fine
async fn _decode_server_online(_srv: crate::Server) {}
