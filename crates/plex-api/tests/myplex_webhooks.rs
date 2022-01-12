use mockito::mock;
use plex_api::{url::MYPLEX_WEBHOOKS_PATH, Error};

#[plex_api_test_helper::async_offline_test]
async fn webhook_free_user(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_WEBHOOKS_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json")
        .expect(0)
        .create();

    let webhook_manager_result = myplex.webhook_manager().await;
    m.assert();

    assert!(
        webhook_manager_result.is_err(),
        "Webhook manager should not be returned for free user"
    );

    let error = webhook_manager_result.err().unwrap();

    assert!(
        matches!(error, Error::SubscriptionFeatureNotAvailable(f) if format!("{}", f) == "Webhooks"),
        "Unexpected error"
    );
}

#[plex_api_test_helper::async_offline_test(plexpass_user)]
async fn webhook_plexpass_user_no_webhooks(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_WEBHOOKS_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json")
        .create();

    let webhook_manager_result = myplex.webhook_manager().await;
    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 0, "Expected no webhooks");

    let m = mock("POST", MYPLEX_WEBHOOKS_PATH)
        .with_status(201)
        .match_body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1")
        .with_body("empty")
        .create();
    let add_result = webhook_manager.add("https://example.com/webhook1").await;
    m.assert();

    add_result.expect("failed to add webhook");

    let current_webhooks = webhook_manager.webhooks();

    assert_eq!(current_webhooks.len(), 1, "Expected single webhook");

    assert_eq!(
        current_webhooks[0].url(),
        "https://example.com/webhook1",
        "Unexpected webhook url"
    );
}

#[plex_api_test_helper::async_offline_test(plexpass_user)]
async fn webhook_plexpass_user_two_webhooks(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_WEBHOOKS_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/webhooks_two.json")
        .create();

    let webhook_manager_result = myplex.webhook_manager().await;

    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 2, "Expected two webhooks");

    let m = mock("POST", MYPLEX_WEBHOOKS_PATH)
        .with_status(201)
        .match_body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1")
        .with_body("empty")
        .create();

    let delete_result = webhook_manager.delete("https://example.com/webhook2").await;
    m.assert();

    delete_result.expect("failed to delete webhook");

    let current_webhooks = webhook_manager.webhooks();

    assert_eq!(current_webhooks.len(), 1, "Expected single webhook");

    assert_eq!(
        current_webhooks[0].url(),
        "https://example.com/webhook1",
        "Unexpected webhook url"
    );

    let m = mock("POST", MYPLEX_WEBHOOKS_PATH)
        .with_status(201)
        .match_body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1")
        .with_body("empty")
        .expect(0)
        .create();

    let delete_result = webhook_manager.delete("https://example.com/webhook2").await;
    m.assert();

    assert!(
        matches!(
            delete_result.err().expect("error was expected"),
            Error::WebhookNotFound(url) if url == "https://example.com/webhook2"
        ),
        "Unexpected error"
    );
}

#[plex_api_test_helper::async_offline_test(plexpass_user)]
async fn webhook_plexpass_user_refresh(myplex: MyPlex) {
    let m = mock("GET", MYPLEX_WEBHOOKS_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json")
        .create();

    let webhook_manager_result = myplex.webhook_manager().await;
    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 0, "Expected no webhooks");

    let m = mock("GET", MYPLEX_WEBHOOKS_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/webhooks_two.json")
        .create();

    let webhook_manager_refresh_result = webhook_manager.refresh().await;
    m.assert();

    webhook_manager_refresh_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 2, "Expected two webhooks");
}
