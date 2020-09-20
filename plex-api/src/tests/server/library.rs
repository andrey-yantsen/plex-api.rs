test_case_online_all!(_get_sections_online);

async fn _get_sections_online(srv: crate::Server) {
    let library = srv.get_sections().await;
    assert!(
        library.is_ok(),
        "Unable to get Sections: {:?}",
        library.err()
    );
}

test_case_online_all!(_get_recently_added_online);

async fn _get_recently_added_online(srv: crate::Server) {
    let library = srv.get_recently_added().await;
    assert!(
        library.is_ok(),
        "Unable to get Recently Added: {:?}",
        library.err()
    );
}

test_case_online_all!(_get_on_deck_online);

async fn _get_on_deck_online(srv: crate::Server) {
    let library = srv.get_on_deck().await;
    assert!(
        library.is_ok(),
        "Unable to get On Deck: {:?}",
        library.err()
    );
}
