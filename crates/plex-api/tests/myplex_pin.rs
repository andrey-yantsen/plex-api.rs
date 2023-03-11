mod fixtures;

mod offline {
    use std::sync::Arc;

    use super::fixtures::offline::{client::*, myplex::*, Mocked};
    use httpmock::Method::{GET, POST, PUT};
    use plex_api::{
        url::{MYPLEX_PINS, MYPLEX_PINS_LINK},
        HttpClient, MyPlex, PinManager,
    };

    #[plex_api_test_helper::offline_test]
    async fn link(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();

        let mut mock = mock_server.mock(|when, then| {
            when.method(PUT)
                .path(MYPLEX_PINS_LINK)
                .x_www_form_urlencoded_tuple("code", "abcd");
            then.status(404)
                .header("content-type", "text/json")
                .body(r#"{"errors":[{"code": 1020, "message": "Code not found or expired"}]}"#);
        });

        let pin_manager = myplex.pin_manager();

        assert!(pin_manager.link("abcd").await.is_err());
        mock.assert();
        mock.delete();

        let mut mock = mock_server.mock(|when, then| {
            when.method(PUT)
                .path(MYPLEX_PINS_LINK)
                .x_www_form_urlencoded_tuple("code", "code");
            then.status(204);
        });

        pin_manager.link("code").await.expect("failed to link");
        mock.assert();
        mock.delete();
    }

    #[plex_api_test_helper::offline_test]
    async fn pin(client_anonymous: Mocked<HttpClient>) {
        let (client, mock_server) = client_anonymous.split();
        let pin_manager = PinManager::new(Arc::new(client));

        let mock_body_new = include_str!("mocks/myplex/api/v2/pins_new.json");

        let mut mock = mock_server.mock(|when, then| {
            when.method(POST).path(MYPLEX_PINS);
            then.status(201)
                .header("content-type", "text/json")
                .body(mock_body_new);
        });

        let pin = pin_manager.pin().await.expect("failed to pin");
        mock.assert();
        assert!(pin.is_expired(), "Pin should be expired");
        pin.check().await.expect_err("check should've failed");
        mock.delete();

        let mut mock = mock_server.mock(|when, then| {
            when.method(POST).path(MYPLEX_PINS);
            then.status(201)
                .header("content-type", "text/json")
                .body(mock_body_new.replace("2022-02-01T00:15:00Z", "2999-01-01T00:00:00Z"));
        });

        let pin = pin_manager.pin().await.expect("failed to pin");
        mock.assert();
        assert!(!pin.is_expired());
        pin.check().await.expect_err("check should've failed");
        mock.delete();

        let mut mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/{}", MYPLEX_PINS, pin.pin.id));
            then.status(200)
                .body_from_file("tests/mocks/myplex/api/v2/pins_linked.json");
        });

        pin.check().await.expect("failed to check pin");

        mock.assert();
        mock.delete();
    }
}
