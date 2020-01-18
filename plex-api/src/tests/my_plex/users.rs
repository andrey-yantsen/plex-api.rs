#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_users_online() {
    use crate::tests::retry::FutureRetryHandler;
    use crate::MyPlexAccount;
    use futures_retry::FutureRetry;
    use std::env;

    let acc = {
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        MyPlexAccount::by_token(&auth_token).await.unwrap()
    };
    let acc = &acc;
    let users = FutureRetry::new(
        move || acc.get_users(),
        FutureRetryHandler::new(5, "Getting users"),
    )
    .await;
    if let Err(e) = users {
        assert!(false, "Unable to get users: {:?}", e);
    }
}
