mod fixtures;

use crate::fixtures::offline::Mocked;
use fixtures::offline::myplex::*;
use httpmock::Method::GET;
use httpmock::Method::POST;
use plex_api::{url::MYPLEX_WEBHOOKS_PATH, Error, MyPlex};

#[plex_api_test_helper::async_offline_test]
async fn webhook_free_user(#[future] myplex: Mocked<MyPlex>) {
    let myplex = myplex.await;
    let (myplex, mock_server) = myplex.split();

    let m = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json");
    });

    let webhook_manager_result = myplex.webhook_manager().await;
    m.assert_hits(0);

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

#[plex_api_test_helper::async_offline_test]
async fn webhook_plexpass_user_no_webhooks(#[future] myplex_plexpass: Mocked<MyPlex>) {
    let myplex = myplex_plexpass.await;
    let (myplex, mock_server) = myplex.split();

    let m = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json");
    });

    let webhook_manager_result = myplex.webhook_manager().await;
    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 0, "Expected no webhooks");

    let m = mock_server.mock(|when, then| {
        when.method(POST)
            .path(MYPLEX_WEBHOOKS_PATH)
            .body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1");
        then.status(201)
            .header("content-type", "text/json")
            .body("");
    });

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

#[plex_api_test_helper::async_offline_test]
async fn webhook_plexpass_user_two_webhooks(#[future] myplex_plexpass: Mocked<MyPlex>) {
    let myplex = myplex_plexpass.await;
    let (myplex, mock_server) = myplex.split();

    let m = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_two.json");
    });

    let webhook_manager_result = myplex.webhook_manager().await;

    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 2, "Expected two webhooks");

    let m = mock_server.mock(|when, then| {
        when.method(POST)
            .path(MYPLEX_WEBHOOKS_PATH)
            .body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1");
        then.status(201)
            .header("content-type", "text/json")
            .body("");
    });

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

    let m = mock_server.mock(|when, then| {
        when.method(POST)
            .path(MYPLEX_WEBHOOKS_PATH)
            .body("urls%5B%5D=https%3A%2F%2Fexample.com%2Fwebhook1");
        then.status(201)
            .header("content-type", "text/json")
            .body("");
    });

    let delete_result = webhook_manager.delete("https://example.com/webhook2").await;
    m.assert_hits(0);

    assert!(
        matches!(
            delete_result.err().expect("error was expected"),
            Error::WebhookNotFound(url) if url == "https://example.com/webhook2"
        ),
        "Unexpected error"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn webhook_plexpass_user_refresh(#[future] myplex_plexpass: Mocked<MyPlex>) {
    let myplex = myplex_plexpass.await;
    let (myplex, mock_server) = myplex.split();

    let mut mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_none.json");
    });

    let webhook_manager_result = myplex.webhook_manager().await;
    mock.assert();
    mock.delete();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 0, "Expected no webhooks");

    let mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_two.json");
    });

    let webhook_manager_refresh_result = webhook_manager.refresh().await;
    mock.assert();

    webhook_manager_refresh_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 2, "Expected two webhooks");
}

#[plex_api_test_helper::async_offline_test]
async fn webhook_erase(#[future] myplex_plexpass: Mocked<MyPlex>) {
    let myplex = myplex_plexpass.await;
    let (myplex, mock_server) = myplex.split();

    let m = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_WEBHOOKS_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/webhooks_two.json");
    });

    let webhook_manager_result = myplex.webhook_manager().await;

    m.assert();

    let mut webhook_manager =
        webhook_manager_result.expect("Webhook manager should be returned for paid user");

    assert_eq!(webhook_manager.webhooks().len(), 2, "Expected two webhooks");

    let m = mock_server.mock(|when, then| {
        when.method(POST).path(MYPLEX_WEBHOOKS_PATH).body("urls=");
        then.status(201)
            .header("content-type", "text/json")
            .body("");
    });

    let delete_result = webhook_manager.set(vec![]).await;
    m.assert();

    delete_result.expect("failed to erase webhooks");
}
