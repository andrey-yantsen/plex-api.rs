mod fixtures;

mod offline {
    use super::fixtures::offline::{myplex::*, Mocked};
    use httpmock::Method::GET;
    use plex_api::{sharing::InviteStatus, url::MYPLEX_INVITES_FRIENDS, MyPlex};

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

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_INVITES_FRIENDS)
                .query_param("includeSharedServers", "true")
                .query_param("includeSharedSources", "true")
                .query_param("includeSharingSettings", "true")
                .query_param("status", "accepted");
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file(mock_file);
        });

        assert_eq!(
            friends_count,
            sharing.friends(InviteStatus::Accepted).await.unwrap().len()
        );

        m.assert();
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
