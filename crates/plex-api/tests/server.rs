mod fixtures;

fn map<I, F, R>(items: &[I], mapper: F) -> Vec<R>
where
    F: FnMut(&I) -> R,
{
    items.iter().map(mapper).collect()
}

mod offline {
    use crate::map;

    use super::fixtures::offline::client::*;
    use super::fixtures::offline::server::*;
    use super::fixtures::offline::Mocked;
    use httpmock::Method::GET;
    use plex_api::{
        url::{MYPLEX_USER_INFO_PATH, SERVER_MEDIA_PROVIDERS},
        HttpClient, Server,
    };
    use plex_api::{Library, MetadataItem};

    #[plex_api_test_helper::offline_test]
    #[case::free("tests/mocks/server/media/providers_free.json")]
    #[case::plexpass("tests/mocks/server/media/providers_plexpass.json")]
    #[case::unclaimed("tests/mocks/server/media/providers_unclaimed.json")]
    async fn load_server(client_authenticated: Mocked<HttpClient>, #[case] mock_file: &str) {
        let (client_authenticated, mock_server) = client_authenticated.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(SERVER_MEDIA_PROVIDERS)
                .header("X-Plex-Token", "fixture_auth_token");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file(mock_file);
        });

        let server_result = Server::new(mock_server.base_url(), client_authenticated).await;

        m.assert();
        server_result.expect("can't recover myplex from server");
    }

    #[plex_api_test_helper::offline_test]
    async fn myplex_recover_from_server(#[future] server_authenticated: Mocked<Server>) {
        let server = server_authenticated.await;
        let (server, mock_server) = server.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_USER_INFO_PATH)
                .header("X-Plex-Token", "fixture_auth_token");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/myplex/api/v2/user/user_info_free_guest.json");
        });

        let plex_result = server
            .myplex()
            .expect("failed to get MyPlex from the server")
            .refresh()
            .await;

        m.assert();
        plex_result.expect("can't recover myplex from server");
    }

    #[plex_api_test_helper::offline_test]
    async fn movie_library(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.await.split();

        let libraries = server.libraries();
        let library = if let Library::Movie(lib) = &libraries[0] {
            lib
        } else {
            panic!("Unexpected library: {:?}", libraries[0]);
        };

        assert_eq!(library.title(), "Movies");

        let mut m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/library/sections/1/all")
                .query_param("type", "1");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/movie_library.json");
        });

        let movies = library.movies().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&movies, |e| e.title().to_owned()),
            vec![
                "Big Buck Bunny",
                "Elephants Dream",
                "Interstate 60",
                "Sintel"
            ]
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn tv_library(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.await.split();

        let libraries = server.libraries();
        let library = if let Library::TV(lib) = &libraries[1] {
            lib
        } else {
            panic!("Unexpected library: {:?}", libraries[0]);
        };

        assert_eq!(library.title(), "TV Shows");

        let mut m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/library/sections/2/all")
                .query_param("type", "2");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/tv_library.json");
        });

        let shows = library.shows().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&shows, |e| e.title().to_owned()),
            vec!["The 100", "Game of Thrones",]
        );

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/22/children");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/tv_seasons.json");
        });

        let seasons = shows[0].seasons().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&seasons, |e| e.title().to_owned()),
            vec!["Season 1", "Season 2",]
        );

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/89/children");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/tv_episodes.json");
        });

        let episodes = seasons[0].episodes().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&episodes, |e| e.title().to_owned()),
            vec!["Pilot", "Earth Skills", "Earth Kills"]
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn photo_library(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.await.split();

        let libraries = server.libraries();
        let library = if let Library::Photo(lib) = &libraries[3] {
            lib
        } else {
            panic!("Unexpected library: {:?}", libraries[0]);
        };

        assert_eq!(library.title(), "Photos");

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/sections/3/all");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/photo_library.json");
        });

        let albums = library.albums().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(map(&albums, |e| e.title().to_owned()), vec!["Cats"]);

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/43/children");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/photo_album.json");
        });

        let items = albums[0].contents().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&items, |e| e.title().to_owned()),
            vec!["Cats in bed", "Picture1"]
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn music_library(#[future] server_anonymous: Mocked<Server>) {
        let (server, mock_server) = server_anonymous.await.split();

        let libraries = server.libraries();
        let library = if let Library::Music(lib) = &libraries[2] {
            lib
        } else {
            panic!("Unexpected library: {:?}", libraries[0]);
        };

        assert_eq!(library.title(), "Music");

        let mut m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/library/sections/5/all")
                .query_param("type", "8");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/music_library.json");
        });

        let artists = library.artists().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&artists, |e| e.title().to_owned()),
            vec!["Skrillex", "System of a Down"]
        );

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/156/children");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/music_albums.json");
        });

        let albums = artists[0].albums().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(map(&albums, |e| e.title().to_owned()), vec!["Try It Out"]);

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/157/children");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/server/media/music_tracks.json");
        });

        let tracks = albums[0].tracks().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec![
                "TRY IT OUT (NEON MIX)",
                "Try It Out (Try Harder Mix)",
                "Try It Out (Put Em Up Mix)"
            ]
        );
    }
}

mod online {
    use crate::map;

    use super::fixtures::online::client::*;
    use super::fixtures::online::server::*;
    use isahc::{config::Configurable, HttpClient as IsahcHttpClient};
    use plex_api::Library;
    use plex_api::MetadataItem;
    use plex_api::Photo;
    use plex_api::PhotoAlbum;
    use plex_api::PhotoAlbumItem;
    use plex_api::Video;
    use plex_api::{HttpClient, HttpClientBuilder, Server};

    #[plex_api_test_helper::online_test]
    async fn load_server(#[future] server: Server) {
        let _ = server.await;
    }

    // Claim/unclaim could take long time due to unknown reasons.
    async fn get_server_with_longer_timeout(server: Server, client: HttpClient) -> Server {
        let server_url = server.client().api_url.clone();

        let client = HttpClientBuilder::from(client)
            .set_http_client(
                IsahcHttpClient::builder()
                    .timeout(std::time::Duration::from_secs(60))
                    .connect_timeout(std::time::Duration::from_secs(30))
                    .redirect_policy(isahc::config::RedirectPolicy::None)
                    .expect_continue(isahc::config::ExpectContinue::disabled())
                    .build()
                    .expect("failed to create testing http client"),
            )
            .build()
            .expect("failed to build client");

        Server::new(server_url, client)
            .await
            .expect("failed to get server")
    }

    #[plex_api_test_helper::online_test]
    async fn list_libraries(#[future] server: Server) {
        let server = server.await;

        let libraries = server.libraries();
        assert_eq!(libraries.len(), 4);
        assert_eq!(libraries[0].title(), "Movies");
        assert_eq!(libraries[1].title(), "TV Shows");
        assert_eq!(libraries[2].title(), "Music");
        assert_eq!(libraries[3].title(), "Photos");
    }

    #[plex_api_test_helper::online_test]
    async fn movies(#[future] server: Server) {
        let server = server.await;
        let libraries = server.libraries();

        let library = if let Library::Movie(lib) = libraries.get(0).unwrap() {
            lib
        } else {
            panic!("Unexpected library type");
        };

        let movies = library.movies().await.unwrap();
        assert_eq!(
            map(&movies, |e| e.title().to_owned()),
            vec![
                "Big Buck Bunny",
                "Elephants Dream",
                "Interstate 60",
                "Sintel"
            ]
        );

        let collections = library.collections().await.unwrap();
        assert_eq!(
            map(&collections, |e| e.title().to_owned()),
            vec!["Animation"]
        );

        let movies = collections[0].children().await.unwrap();
        assert_eq!(
            map(&movies, |e| e.title().to_owned()),
            vec!["Elephants Dream", "Big Buck Bunny", "Sintel"]
        );

        let playlists = library.playlists().await.unwrap();
        assert_eq!(
            map(&playlists, |e| e.title().to_owned()),
            vec!["Movies Since 2007", "Mixed", "Some Movies"]
        );

        let videos = playlists[0].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec!["Big Buck Bunny", "Sintel"]
        );
        assert!(matches!(videos[0], Video::Movie(_)));
        assert!(matches!(videos[1], Video::Movie(_)));

        let videos = playlists[1].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec![
                "Big Buck Bunny",
                "What is Dead May Never Die",
                "The North Remembers"
            ]
        );
        assert!(matches!(videos[0], Video::Movie(_)));
        assert!(matches!(videos[1], Video::Episode(_)));
        assert!(matches!(videos[2], Video::Episode(_)));

        let videos = playlists[2].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec!["Interstate 60", "Sintel"]
        );
        assert!(matches!(videos[0], Video::Movie(_)));
        assert!(matches!(videos[1], Video::Movie(_)));
    }

    #[plex_api_test_helper::online_test]
    async fn tv(#[future] server: Server) {
        let server = server.await;
        let libraries = server.libraries();

        let library = if let Library::TV(lib) = libraries.get(1).unwrap() {
            lib
        } else {
            panic!("Unexpected library type");
        };

        let shows = library.shows().await.unwrap();
        assert_eq!(
            map(&shows, |e| e.title().to_owned()),
            vec!["The 100", "Game of Thrones"]
        );

        let seasons = shows[0].seasons().await.unwrap();
        assert_eq!(
            map(&seasons, |e| e.title().to_owned()),
            vec!["Season 1", "Season 2"]
        );
        assert_eq!(seasons[0].season_number(), Some(1));
        assert_eq!(seasons[1].season_number(), Some(2));

        assert_eq!(seasons[0].show().await.unwrap().unwrap().title(), "The 100");

        let episodes = seasons[0].episodes().await.unwrap();
        assert_eq!(
            map(&episodes, |e| e.title().to_owned()),
            vec![
                "Pilot",
                "Earth Skills",
                "Earth Kills",
                "Murphy's Law",
                "Twilight's Last Gleaming",
                "His Sister's Keeper",
                "Contents Under Pressure",
                "Day Trip",
                "Unity Day",
            ]
        );
        assert_eq!(
            map(&episodes, |e| e.episode_number()),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );

        assert_eq!(
            episodes[0].season().await.unwrap().unwrap().title(),
            "Season 1"
        );
        assert_eq!(
            episodes[0]
                .season()
                .await
                .unwrap()
                .unwrap()
                .show()
                .await
                .unwrap()
                .unwrap()
                .title(),
            "The 100"
        );

        let episodes = seasons[1].episodes().await.unwrap();
        assert_eq!(
            map(&episodes, |e| e.title().to_owned()),
            vec![
                "The 48",
                "Inclement Weather",
                "Reapercussions",
                "Many Happy Returns",
                "Human Trials",
                "Fog of War",
                "Long Into an Abyss",
                "Spacewalker",
                "Remember Me",
            ]
        );
        assert_eq!(
            map(&episodes, |e| e.episode_number()),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );

        let seasons = shows[1].seasons().await.unwrap();
        assert_eq!(
            map(&seasons, |e| e.title().to_owned()),
            vec!["Season 1", "Season 2"]
        );
        assert_eq!(seasons[0].season_number(), Some(1));
        assert_eq!(seasons[1].season_number(), Some(2));

        let episodes = seasons[0].episodes().await.unwrap();
        assert_eq!(
            map(&episodes, |e| e.title().to_owned()),
            vec![
                "Winter Is Coming",
                "The Kingsroad",
                "Lord Snow",
                "Cripples, Bastards, and Broken Things",
                "The Wolf and the Lion",
                "A Golden Crown",
                "You Win or You Die",
                "The Pointy End",
                "Baelor",
            ]
        );
        assert_eq!(
            map(&episodes, |e| e.episode_number()),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );

        let episodes = seasons[1].episodes().await.unwrap();
        assert_eq!(
            map(&episodes, |e| e.title().to_owned()),
            vec![
                "The North Remembers",
                "The Night Lands",
                "What is Dead May Never Die",
                "Garden of Bones",
                "The Ghost of Harrenhal",
                "The Old Gods and the New",
                "A Man Without Honor",
                "The Prince of Winterfell",
                "Blackwater",
            ]
        );
        assert_eq!(
            map(&episodes, |e| e.episode_number()),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9)
            ]
        );

        let collections = library.collections().await.unwrap();
        assert_eq!(map(&collections, |e| e.title().to_owned()), vec!["SciFi"]);

        let shows = collections[0].children().await.unwrap();
        assert_eq!(map(&shows, |e| e.title().to_owned()), vec!["The 100"]);

        let playlists = library.playlists().await.unwrap();
        assert_eq!(
            map(&playlists, |e| e.title().to_owned()),
            vec!["Episodes with H", "Mixed", "Pilot"]
        );

        let videos = playlists[0].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec![
                "The Ghost of Harrenhal",
                "His Sister's Keeper",
                "Human Trials",
                "A Man Without Honor",
                "Many Happy Returns"
            ]
        );
        assert!(matches!(videos[0], Video::Episode(_)));
        assert!(matches!(videos[1], Video::Episode(_)));
        assert!(matches!(videos[2], Video::Episode(_)));
        assert!(matches!(videos[3], Video::Episode(_)));
        assert!(matches!(videos[4], Video::Episode(_)));

        let videos = playlists[1].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec![
                "Big Buck Bunny",
                "What is Dead May Never Die",
                "The North Remembers"
            ]
        );
        assert!(matches!(videos[0], Video::Movie(_)));
        assert!(matches!(videos[1], Video::Episode(_)));
        assert!(matches!(videos[2], Video::Episode(_)));

        let videos = playlists[2].children().await.unwrap();
        assert_eq!(
            map(&videos, |e| e.title().to_owned()),
            vec!["Pilot", "The 48", "Winter Is Coming", "The North Remembers"]
        );
        assert!(matches!(videos[0], Video::Episode(_)));
        assert!(matches!(videos[1], Video::Episode(_)));
        assert!(matches!(videos[2], Video::Episode(_)));
        assert!(matches!(videos[3], Video::Episode(_)));
    }

    #[plex_api_test_helper::online_test]
    async fn music(#[future] server: Server) {
        let server = server.await;
        let libraries = server.libraries();

        let library = if let Library::Music(lib) = libraries.get(2).unwrap() {
            lib
        } else {
            panic!("Unexpected library type");
        };

        let artists = library.artists().await.unwrap();
        assert_eq!(
            map(&artists, |e| e.title().to_owned()),
            vec!["Skrillex", "System of a Down"]
        );

        let albums = artists[0].albums().await.unwrap();
        assert_eq!(map(&albums, |e| e.title().to_owned()), vec!["Try It Out"]);

        let tracks = albums[0].tracks().await.unwrap();
        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec![
                "TRY IT OUT (NEON MIX)",
                "Try It Out (Try Harder Mix)",
                "Try It Out (Put Em Up Mix)"
            ]
        );

        let album = tracks[0].album().await.unwrap().unwrap();
        assert_eq!(album.title(), "Try It Out");
        let artist = album.artist().await.unwrap().unwrap();
        assert_eq!(artist.title(), "Skrillex");

        let albums = artists[1].albums().await.unwrap();
        assert_eq!(
            map(&albums, |e| e.title().to_owned()),
            vec!["Aerials [MCD]", "Toxicity"]
        );

        let tracks = albums[0].tracks().await.unwrap();
        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec!["Aerials", "Streamline (album version)", "Sugar (live)"]
        );

        let tracks = albums[1].tracks().await.unwrap();
        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec!["Toxicity", "Marmalade", "Metro"]
        );

        let playlists = library.playlists().await.unwrap();
        assert_eq!(
            map(&playlists, |e| e.title().to_owned()),
            vec!["Best Music"]
        );

        let tracks = playlists[0].children().await.unwrap();
        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec!["Streamline (album version)", "Metro"]
        );
    }

    #[plex_api_test_helper::online_test]
    async fn photos(#[future] server: Server) {
        let server = server.await;
        let libraries = server.libraries();

        let library = if let Library::Photo(lib) = libraries.get(3).unwrap() {
            lib
        } else {
            panic!("Unexpected library type");
        };

        let albums = library.albums().await.unwrap();
        assert_eq!(map(&albums, |e| e.title().to_owned()), vec!["Cats"]);

        fn split(contents: Vec<PhotoAlbumItem>) -> (Vec<PhotoAlbum>, Vec<Photo>) {
            let mut albums = Vec::new();
            let mut photos = Vec::new();

            for item in contents {
                match item {
                    PhotoAlbumItem::PhotoAlbum(a) => {
                        albums.push(a);
                    }
                    PhotoAlbumItem::Photo(p) => {
                        photos.push(p);
                    }
                }
            }

            (albums, photos)
        }

        let (albums, photos) = split(albums[0].contents().await.unwrap());
        assert_eq!(
            map(&albums, |e| e.title().to_owned()),
            vec!["Cats in bed", "Cats not in bed"]
        );
        assert_eq!(
            map(&photos, |e| e.title().to_owned()),
            vec!["Picture1", "Picture2", "Picture3"]
        );

        let (inner_albums, photos) = split(albums[0].contents().await.unwrap());
        assert!(inner_albums.is_empty());
        assert_eq!(
            map(&photos, |e| e.title().to_owned()),
            vec!["Picture1", "Picture2", "Picture3"]
        );

        let (inner_albums, photos) = split(albums[1].contents().await.unwrap());
        assert!(inner_albums.is_empty());
        assert_eq!(
            map(&photos, |e| e.title().to_owned()),
            vec!["Picture1", "Picture2", "Picture3"]
        );

        let parent = photos[0].album().await.unwrap().unwrap();
        assert_eq!(parent.title(), "Cats not in bed");

        let playlists = library.playlists().await.unwrap();
        assert_eq!(map(&playlists, |e| e.title().to_owned()), vec!["Cats"]);

        let tracks = playlists[0].children().await.unwrap();
        assert_eq!(
            map(&tracks, |e| e.title().to_owned()),
            vec!["Picture1", "Picture2", "Picture3", "Picture1", "Picture2", "Picture3"]
        );
    }

    #[allow(unused_attributes)]
    #[plex_api_test_helper::online_test_claimed_server]
    #[ignore = "Must be run manually"]
    async fn claim_server(#[future] server_unclaimed: Server, client_authenticated: HttpClient) {
        let server =
            get_server_with_longer_timeout(server_unclaimed.await, client_authenticated).await;

        let myplex = server
            .myplex()
            .expect("failed to get MyPlex from the server");
        let claim_token = myplex
            .claim_token()
            .await
            .expect("failed to get claim token")
            .to_string();

        server
            .claim(&claim_token)
            .await
            .expect("failed to claim server")
            .unclaim()
            .await
            .expect("failed to unclaim server");
    }

    #[allow(unused_attributes)]
    #[plex_api_test_helper::online_test_claimed_server]
    #[ignore = "Must be run manually"]
    async fn unclaim_server(#[future] server_claimed: Server) {
        let server = server_claimed.await;
        let client = server.client().to_owned();

        let server = get_server_with_longer_timeout(server, client).await;

        server.unclaim().await.expect("failed to unclaim server");
    }
}
