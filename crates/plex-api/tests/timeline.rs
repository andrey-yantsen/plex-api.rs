mod fixtures;

mod offline {
    use super::fixtures::offline::{server::*, Mocked};

    use httpmock::Method::GET;
    use plex_api::{
        library::{MetadataItem, Movie},
        Server,
    };

    #[plex_api_test_helper::offline_test]
    async fn timeline(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.split();

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/182");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/timeline/metadata_182_1.json");
        });

        let movie: Movie = server.item_by_id("182").await.unwrap().try_into().unwrap();
        m.assert();
        m.delete();

        let metadata = movie.metadata();

        // Movie hasn't been fully viewed yet.
        assert_eq!(metadata.view_count, None);
        // Movie has been played up to this point.
        assert_eq!(metadata.view_offset, Some(70000));

        let mut s = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/:/timeline")
                .query_param("key", "/library/metadata/182")
                .query_param("ratingKey", "182")
                .query_param("offline", "1")
                .query_param("state", "playing")
                .query_param("time", "75000");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/timeline/timeline.json");
        });
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/182");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/timeline/metadata_182_2.json");
        });

        let movie: Movie = server.update_timeline(&movie, 75000).await.unwrap();
        s.assert();
        s.delete();
        m.assert();
        m.delete();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, None);
        assert_eq!(metadata.view_offset, Some(75000));

        let mut s = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/:/scrobble")
                .query_param("key", "182")
                .query_param("identifier", "com.plexapp.plugins.library");
            then.status(200);
        });
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/182");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/timeline/metadata_182_3.json");
        });

        let movie: Movie = server.mark_watched(&movie).await.unwrap();
        s.assert();
        s.delete();
        m.assert();
        m.delete();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, Some(1));
        assert_eq!(metadata.view_offset, None);

        let mut s = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/:/unscrobble")
                .query_param("key", "182")
                .query_param("identifier", "com.plexapp.plugins.library");
            then.status(200);
        });
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/182");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/timeline/metadata_182_4.json");
        });

        let movie: Movie = server.mark_unwatched(&movie).await.unwrap();
        s.assert();
        s.delete();
        m.assert();
        m.delete();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, None);
        assert_eq!(metadata.view_offset, None);
    }
}

mod online {
    use super::fixtures::online::server::*;
    use plex_api::{library::MetadataItem, library::Movie, Server};

    #[plex_api_test_helper::online_test]
    async fn timeline(#[future] server: Server) {
        let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();

        let metadata = movie.metadata();

        // Movie has been fully viewed.
        assert_eq!(metadata.view_count, Some(1));
        // Playback is not in progress.
        assert_eq!(metadata.view_offset, None);

        let movie = server.mark_unwatched(&movie).await.unwrap();

        let metadata = movie.metadata();

        // Movie has not been viewed.
        assert_eq!(metadata.view_count, None);
        // Playback is not in progress.
        assert_eq!(metadata.view_offset, None);

        let movie = server.mark_watched(&movie).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, Some(1));
        assert_eq!(metadata.view_offset, None);

        let movie = server.mark_watched(&movie).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, Some(2));
        assert_eq!(metadata.view_offset, None);

        let movie: Movie = server.item_by_id("182").await.unwrap().try_into().unwrap();

        let metadata = movie.metadata();

        // Movie hasn't been fully viewed yet.
        assert_eq!(metadata.view_count, None);
        // Movie has been played up to this point.
        assert_eq!(metadata.view_offset, Some(70000));

        let movie: Movie = server.update_timeline(&movie, 260000).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, None);
        assert_eq!(metadata.view_offset, Some(260000));

        let movie: Movie = server.update_timeline(&movie, 90000).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, None);
        assert_eq!(metadata.view_offset, Some(90000));

        let movie: Movie = server.mark_watched(&movie).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, Some(1));
        assert_eq!(metadata.view_offset, None);

        let movie: Movie = server.update_timeline(&movie, 120000).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, Some(1));
        assert_eq!(metadata.view_offset, Some(120000));

        let movie: Movie = server.mark_unwatched(&movie).await.unwrap();

        let metadata = movie.metadata();

        assert_eq!(metadata.view_count, None);
        assert_eq!(metadata.view_offset, None);
    }
}
