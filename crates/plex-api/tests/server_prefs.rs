mod fixtures;

mod offline {
    use super::fixtures::offline::{server::*, Mocked};
    use httpmock::Method::{GET, PUT};
    use plex_api::{url::SERVER_PREFS, Server};

    #[plex_api_test_helper::offline_test]
    async fn load_prefs(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.await.split();

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path(SERVER_PREFS);
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/prefs.json");
        });

        let mut prefs = server
            .preferences()
            .await
            .expect("failed to load preferences");

        m.assert();
        m.delete();

        prefs
            .set("CinemaTrailersType", plex_api::SettingValue::Int(0))
            .expect("failed to update value");

        let m = mock_server.mock(|when, then| {
            when.method(PUT)
                .path(SERVER_PREFS)
                .query_param("CinemaTrailersType", "0");
            then.status(200).header("content-type", "text/json");
        });

        prefs
            .commit()
            .await
            .expect("failed to commit updated preferences");

        m.assert();
    }
}

mod online {
    use std::time::Duration;

    use super::fixtures::online::server::*;
    use plex_api::Server;

    #[plex_api_test_helper::online_test_non_shared_server]
    async fn load_prefs(#[future] server: Server) {
        let server = server.await;
        let mut prefs = server
            .preferences()
            .await
            .expect("failed to load preferences");

        let current = {
            let s = prefs
                .get("ButlerTaskDeepMediaAnalysis")
                .expect("failed to get value");

            match s.value {
                plex_api::SettingValue::Bool(b) => b,
                _ => panic!("expected bool value"),
            }
        };

        prefs
            .set(
                "ButlerTaskDeepMediaAnalysis",
                plex_api::SettingValue::Bool(!current),
            )
            .expect("failed to update value");

        prefs
            .commit()
            .await
            .expect("failed to commit updated preferences");

        // Server responds with 400 Bad Request if the next request is
        // sent too soon after the PUT, so let's add some retries
        async_std::task::sleep(Duration::from_millis(500)).await;
        let mut prefs = server.preferences().await;

        for _ in 0..10 {
            async_std::task::sleep(Duration::from_millis(500)).await;
            prefs = server.preferences().await;

            if prefs.is_ok() {
                break;
            }
        }

        let prefs = prefs.expect("failed to load preferences 2nd time");

        let new_value = {
            let s = prefs
                .get("ButlerTaskDeepMediaAnalysis")
                .expect("failed to get value");

            match s.value {
                plex_api::SettingValue::Bool(b) => b,
                _ => panic!("expected bool value"),
            }
        };

        assert_eq!(!current, new_value, "Value did not change");
    }
}
