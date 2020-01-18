#[cfg(feature = "test_connect_anonymous")]
use crate::tests::get_server_anonymous;
#[cfg(feature = "test_connect_authenticated")]
use crate::tests::get_server_authenticated;

#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_settings_online_authenticated() {
    let srv = get_server_authenticated().await;
    let settings = srv.get_settings().await;
    assert!(
        settings.is_ok(),
        "Unable to get settings: {:?}",
        settings.err()
    );
}

#[cfg(feature = "test_connect_anonymous")]
#[tokio::test]
async fn decode_settings_online_anonymous() {
    let srv = get_server_anonymous().await;
    let settings = srv.get_settings().await;
    assert!(
        settings.is_ok(),
        "Unable to get settings: {:?}",
        settings.err()
    );
}
