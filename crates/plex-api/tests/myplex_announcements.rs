mod fixtures;

mod offline {
    use super::fixtures::offline::{client::*, Mocked};
    use httpmock::Method::{GET, PUT};
    use plex_api::{url::MYPLEX_ANNOUNCEMENTS, Error, HttpClient, MyPlex};

    #[plex_api_test_helper::offline_test]
    async fn list_announcements(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET).path(MYPLEX_ANNOUNCEMENTS);
            then.status(200)
                .header("content-type", "application/xml")
                .body_from_file("tests/mocks/myplex/api/announcements.xml");
        });

        let plex = MyPlex::new(client_anonymous);
        let announcements_manager = plex.announcements().await;

        m.assert();

        let announcements_manager = announcements_manager.unwrap();

        let announcements = announcements_manager.announcements();

        assert_eq!(4, announcements.len());

        let first_announcement = &announcements[0];

        assert_eq!(78, first_announcement.id);
    }

    #[plex_api_test_helper::offline_test]
    async fn read_announcement(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET).path(MYPLEX_ANNOUNCEMENTS);
            then.status(200)
                .header("content-type", "application/xml")
                .body_from_file("tests/mocks/myplex/api/announcements.xml");
        });

        let plex = MyPlex::new(client_anonymous);
        let announcements_manager = plex.announcements().await;
        m.assert();

        let mut announcements_manager = announcements_manager.unwrap();
        let announcements = announcements_manager.announcements_mut();

        let first_announcement = &mut announcements[0];
        assert_eq!(78, first_announcement.id);

        let mut m = mock_server.mock(|when, then| {
            when.method(PUT)
                .path(format!("{MYPLEX_ANNOUNCEMENTS}/{}", first_announcement.id))
                .query_param("read", "1");
            then.status(200)
                .header("content-type", "application/xml")
                .body(r#"<Response code="200" status="Updated announcement status"/>"#);
        });

        let tmp = first_announcement.read().await;
        m.assert();
        m.delete();

        tmp.unwrap();

        let m = mock_server.mock(|when, then| {
            when.method(PUT)
                .path(format!("{MYPLEX_ANNOUNCEMENTS}/{}", first_announcement.id))
                .query_param("read", "0");
            then.status(200)
                .header("content-type", "application/xml")
                .body(r#"<Response code="400" status="Some error"/>"#);
        });

        let tmp = first_announcement.unread().await;
        m.assert();

        let err = tmp.unwrap_err();

        assert!(matches!(err, Error::MyPlexApiError { code: 400, .. }));
    }
}

mod online {
    use super::fixtures::online::myplex;
    use plex_api::MyPlex;

    #[plex_api_test_helper::online_test_myplex]
    async fn list_announcements(#[future] myplex: MyPlex) {
        let myplex = myplex.await;
        myplex.announcements().await.unwrap();
    }
}
