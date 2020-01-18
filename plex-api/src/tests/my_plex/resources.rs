#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_resources_online() {
    use crate::tests::retry::FutureRetryHandler;
    use crate::MyPlexAccount;
    use futures_retry::FutureRetry;
    use std::env;

    let auth_token = &env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
    let acc = FutureRetry::new(
        move || MyPlexAccount::by_token(auth_token),
        FutureRetryHandler::new(5, "Log-in by token"),
    )
    .await
    .unwrap();
    let acc = &acc;
    let resources = FutureRetry::new(
        move || acc.get_resources(),
        FutureRetryHandler::new(5, "Getting resources"),
    )
    .await;
    if let Err(e) = resources {
        assert!(false, "Unable to get resources: {:?}", e);
    }
}
