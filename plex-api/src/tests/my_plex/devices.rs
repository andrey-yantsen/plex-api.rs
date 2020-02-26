#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_devices_online() {
    use crate::tests::retry::FutureRetryHandler;
    use crate::MyPlexAccount;
    use futures_retry::FutureRetry;
    use std::env;

    let auth_token = &env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
    let (acc, _) = FutureRetry::new(
        move || MyPlexAccount::by_token(auth_token),
        FutureRetryHandler::new(5, "Log-in by token"),
    )
    .await
    .unwrap();
    let acc = &acc;
    let (_devices, _) = FutureRetry::new(
        move || acc.get_devices(),
        FutureRetryHandler::new(5, "Getting devices"),
    )
    .await
    .unwrap();
}
