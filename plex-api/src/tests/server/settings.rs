test_case_online_all!(_decode_settings_online);

async fn _decode_settings_online(srv: crate::Server) {
    let settings = srv.get_settings().await;
    assert!(
        settings.is_ok(),
        "Unable to get settings: {:?}",
        settings.err()
    );
}
