mod fixtures;

mod offline {
    use super::fixtures::offline::{myplex::*, Mocked};
    use httpmock::{Method::GET, Mock, MockServer};
    use plex_api::{
        sharing::InviteStatus, url::MYPLEX_INVITES_FRIENDS, MyPlex, RestrictionProfile,
    };

    fn prepare_friends_mock<'a>(mock_server: &'a MockServer, mock_file: &'a str) -> Mock<'a> {
        mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_INVITES_FRIENDS)
                .query_param("includeSharedServers", "true")
                .query_param("includeSharedSources", "true")
                .query_param("includeSharingSettings", "true")
                .query_param("status", "accepted");
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file(mock_file);
        })
    }

    #[plex_api_test_helper::offline_test]
    #[case::no_friends("tests/mocks/myplex/api/v2/friends_empty.json", 0)]
    #[case::single_managed_user(
        "tests/mocks/myplex/api/v2/friends_accepted_one_restricted.json",
        1
    )]
    #[case::four_managed_users(
        "tests/mocks/myplex/api/v2/friends_accepted_four_restricted.json",
        4
    )]
    #[case::single_external_user("tests/mocks/myplex/api/v2/friends_accepted_one_external.json", 1)]
    #[case::managed_and_external_user("tests/mocks/myplex/api/v2/friends_accepted_mixed.json", 2)]
    async fn list_friends(
        #[future] myplex: Mocked<MyPlex>,
        #[case] mock_file: &str,
        #[case] friends_count: usize,
    ) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing();

        let m = prepare_friends_mock(&mock_server, mock_file);

        assert_eq!(
            friends_count,
            sharing.friends(InviteStatus::Accepted).await.unwrap().len()
        );

        m.assert();
    }

    #[plex_api_test_helper::offline_test]
    async fn test_filters_deserialization(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing();

        let mut m = prepare_friends_mock(
            &mock_server,
            "tests/mocks/myplex/api/v2/friends_accepted_four_restricted.json",
        );
        let friends = sharing.friends(InviteStatus::Accepted).await.unwrap();
        m.assert();
        m.delete();

        let mut found_adult = false;
        let mut found_little_kid = false;
        let mut found_teen = false;
        let mut found_older_kid = false;

        for friend in friends {
            let filter_movies = friend
                .sharing_settings
                .as_ref()
                .unwrap()
                .filter_movies
                .as_ref()
                .unwrap();

            match &friend.restriction_profile {
                None => {
                    found_adult = true;

                    assert_eq!(vec!["12"], filter_movies.content_rating);
                    assert_eq!(vec!["Approved"], filter_movies.exclude_content_rating);
                    assert_eq!(
                        vec!["BAFTA", "1001 Movies You Must See Before You Die"],
                        filter_movies.label
                    );
                    assert!(filter_movies.exclude_label.is_empty());
                }
                Some(RestrictionProfile::LittleKid) => {
                    found_little_kid = true;

                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-Y"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "TV-G"));
                    assert!(!filter_movies.content_rating.iter().any(|r| r == "US/PG"));
                    assert!(!filter_movies.content_rating.iter().any(|r| r == "US/PG-13"));
                    assert!(!filter_movies.content_rating.iter().any(|r| r == "US/TV-Y7"));
                    assert!(filter_movies.exclude_content_rating.is_empty());
                    assert!(filter_movies.label.is_empty());
                    assert!(filter_movies.exclude_label.is_empty());
                }
                Some(RestrictionProfile::OlderKid) => {
                    found_older_kid = true;

                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-Y"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-Y7"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-PG"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "TV-G"));
                    assert!(!filter_movies.content_rating.iter().any(|r| r == "US/PG-13"));
                    assert!(filter_movies.exclude_content_rating.is_empty());
                    assert!(filter_movies.label.is_empty());
                    assert!(filter_movies.exclude_label.is_empty());
                }
                Some(RestrictionProfile::Teen) => {
                    found_teen = true;

                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-Y"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-Y7"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/TV-PG"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "TV-G"));
                    assert!(filter_movies.content_rating.iter().any(|r| r == "US/PG-13"));
                    assert!(filter_movies.exclude_content_rating.is_empty());
                    assert!(filter_movies.label.is_empty());
                    assert!(filter_movies.exclude_label.is_empty());
                }
                _ => {
                    panic!("Unexpected friend");
                }
            }
        }

        assert!(
            found_adult,
            "'Adult' friend wasn't found in the friends list"
        );
        assert!(
            found_little_kid,
            "'Little kid' friend wasn't found in the friends list"
        );
        assert!(found_teen, "'Teen' friend wasn't found in the friends list");
        assert!(
            found_older_kid,
            "'Older kid' friend wasn't found in the friends list"
        );
    }
}

mod online {
    use super::fixtures::online::myplex;
    use plex_api::{sharing::InviteStatus, MyPlex};

    #[plex_api_test_helper::online_test_myplex]
    async fn list_friends(#[future] myplex: MyPlex) {
        let myplex = myplex.await;
        myplex
            .sharing()
            .friends(InviteStatus::Accepted)
            .await
            .unwrap();
    }
}
