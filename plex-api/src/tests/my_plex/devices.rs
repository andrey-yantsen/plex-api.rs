#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_devices_online() {
    use crate::tests::retry::FutureRetryHandler;
    use crate::MyPlexAccount;
    use futures_retry::FutureRetry;
    use std::env;

    let acc = {
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        MyPlexAccount::by_token(&auth_token).await.unwrap()
    };
    let acc = &acc;
    let devices = FutureRetry::new(
        move || acc.get_devices(),
        FutureRetryHandler::new(5, "Getting devices"),
    )
    .await;
    if let Err(e) = devices {
        assert!(false, "Unable to get devices: {:?}", e);
    } else {
        assert!(true);
    }
}
