mod fixtures;

mod offline {
    use super::fixtures::offline::{myplex::*, Mocked};
    use httpmock::{
        Method::{DELETE, GET, POST},
        Mock, MockServer,
    };
    use plex_api::{
        sharing::{Filters, InviteStatus, Permissions, ShareableLibrary, User},
        url::{
            MYPLEX_INVITES_FRIENDS, MYPLEX_INVITES_INVITE, MYPLEX_INVITES_SHARED_SERVERS,
            MYPLEX_SERVERS,
        },
        MyPlex, RestrictionProfile,
    };

    fn prepare_friends_mock<'a>(
        mock_server: &'a MockServer,
        status: &'a str,
        mock_file: &'a str,
    ) -> Mock<'a> {
        mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_INVITES_FRIENDS)
                .query_param("includeSharedServers", "true")
                .query_param("includeSharedSources", "true")
                .query_param("includeSharingSettings", "true")
                .query_param("status", status);
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
        let sharing = myplex.sharing().unwrap();

        let m = prepare_friends_mock(&mock_server, "accepted", mock_file);

        assert_eq!(
            friends_count,
            sharing.friends(InviteStatus::Accepted).await.unwrap().len()
        );

        m.assert();
    }

    #[plex_api_test_helper::offline_test]
    async fn filters_deserialization(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing().unwrap();

        let mut m = prepare_friends_mock(
            &mock_server,
            "accepted",
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
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
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

    #[plex_api_test_helper::offline_test]
    async fn invite(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing().unwrap();

        let m = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_INVITES_INVITE)
                .query_param("identifier", "user");
            then.status(201)
                .header("content-type", "application/json")
                .body_from_file("tests/mocks/myplex/api/v2/friends/invite_create.json");
        });

        let friend = sharing
            .invite(plex_api::sharing::User::UsernameOrEmail("user"))
            .await
            .unwrap();

        m.assert();

        assert_eq!(1, friend.id);
        assert_eq!("deadbeef", friend.uuid);
        assert_eq!("username", friend.title);
        assert_eq!("username", friend.username.unwrap());
        assert!(!friend.restricted);
        assert_eq!(None, friend.friendly_name);
        assert_eq!(
            "https://plex.tv/users/deadbeef/avatar?c=23423423",
            friend.thumb
        );
        assert!(!friend.home);
        assert!(matches!(friend.status, Some(InviteStatus::Pending)));
    }

    #[plex_api_test_helper::offline_test]
    async fn share_server(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing().unwrap();

        let server_info_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{MYPLEX_SERVERS}/machine_id"));
            then.status(201)
                .header("content-type", "application/json")
                .body_from_file("tests/mocks/myplex/api/v2/servers/machine_id.json");
        });

        let share_create_mock = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_INVITES_SHARED_SERVERS)
                .json_body(serde_json::json!({
                    "invitedEmail": "user",
                    "librarySectionIds": [1001, 1002],
                    "settings": {
                        "allowChannels": true,
                        "allowSubtitleAdmin": true,
                        "allowSync": true,
                        "allowCameraUpload": false,
                        "allowTuners": 0,
                        "filterMovies": "",
                        "filterMusic": "",
                        "filterTelevision": "",
                        "filterPhotos": null,
                        "filterAll": null
                    },
                    "machineIdentifier": "machine_id"
                }));
            then.status(201)
                .header("content-type", "application/json")
                .body_from_file("tests/mocks/myplex/api/v2/shared_server_create.json");
        });

        let share = sharing
            .share(
                User::UsernameOrEmail("user"),
                plex_api::sharing::ShareableServer::MachineIdentifier("machine_id"),
                &[
                    ShareableLibrary::LibraryId("1"),
                    ShareableLibrary::LibraryId("2"),
                ],
                Permissions::default(),
                Filters::default(),
            )
            .await
            .unwrap();

        server_info_mock.assert();
        share_create_mock.assert();

        assert_eq!("deadbeef1", share.invited.unwrap().uuid);
    }

    #[plex_api_test_helper::offline_test]
    async fn accept_not_pending_forbidden(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing().unwrap();

        let mut m = prepare_friends_mock(
            &mock_server,
            "accepted",
            "tests/mocks/myplex/api/v2/friends_accepted_one_external.json",
        );
        let mut friends = sharing.friends(InviteStatus::Accepted).await.unwrap();
        m.assert();
        m.delete();

        let friend = friends.pop().unwrap();

        let err = friend.accept().await.unwrap_err();
        assert!(
            matches!(err, plex_api::Error::InviteAcceptingNotPendingReceived),
            "Unexpected errors returned by Friend::accept()"
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn accept_and_reject(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();
        let sharing = myplex.sharing().unwrap();

        let mut m = prepare_friends_mock(
            &mock_server,
            "pending_received",
            "tests/mocks/myplex/api/v2/friends_pending_received_one_external.json",
        );
        let mut friends = sharing
            .friends(InviteStatus::PendingReceived)
            .await
            .unwrap();
        m.assert();
        m.delete();

        let friend = friends.pop().unwrap();

        let mock_delete = mock_server.mock(|when, then| {
            when.method(DELETE)
                .path(format!("{MYPLEX_INVITES_FRIENDS}/{}", friend.id));
            then.status(200).header("content-type", "application/json");
        });

        let del = friend.delete().await;
        mock_delete.assert();
        del.unwrap();

        let mut m = prepare_friends_mock(
            &mock_server,
            "pending_received",
            "tests/mocks/myplex/api/v2/friends_pending_received_one_external.json",
        );
        let friends = sharing.friends(InviteStatus::PendingReceived).await;
        m.assert();
        m.delete();

        let mut friends = friends.unwrap();

        let friend = friends.pop().unwrap();

        let mock_accept = mock_server.mock(|when, then| {
            when.method(POST)
                .path(format!("{MYPLEX_INVITES_FRIENDS}/{}/accept", friend.id));
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("tests/mocks/myplex/api/v2/friends/accept.json");
        });

        let friend = friend.accept().await;
        mock_accept.assert();
        let friend = friend.unwrap();

        assert!(
            matches!(friend.status, Some(InviteStatus::Accepted)),
            "Unexpected friend status"
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
            .unwrap()
            .friends(InviteStatus::Accepted)
            .await
            .unwrap();
    }
}
