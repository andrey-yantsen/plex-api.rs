mod fixtures;

mod online {
    use super::fixtures::online::server::*;
    use plex_api::{
        library::{Episode, MetadataItem},
        media_container::server::Feature,
        Server,
    };

    #[plex_api_test_helper::online_test_claimed_server]
    async fn tv_shows(#[future] server_claimed: Server) {
        // Ensure that the test media has chapters and the appropriate markers

        let episode: Episode = server_claimed
            .item_by_id("178")
            .await
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(episode.metadata().chapters.len(), 5);

        if !server_claimed
            .media_container
            .owner_features
            .contains(&Feature::IntroMarkers)
        {
            return;
        }

        assert_eq!(episode.metadata().markers.len(), 1);
    }
}
