test_case_online_all!(
    _directory_populated,
    directory_populated_anonymous,
    directory_populated_authenticated
);

async fn _directory_populated(srv: crate::Server) {
    let library = srv.get_library().await;
    assert!(
        library.is_ok(),
        "Unable to get library: {:?}",
        library.err()
    );

    let library = library.unwrap();

    let directory = library.get_directory();
    assert!(directory.is_some(), "Directory not found");

    let directory = directory.as_ref().unwrap();
    assert!(!directory.is_empty(), "Directory is empty");

    let mut m: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    for d in directory {
        m.insert(&d.key, &d.title);
    }

    assert!(
        m.contains_key("sections"),
        "Key 'sections'' not found in the directory: {:#?}",
        m
    );
    assert!(
        m.contains_key("recentlyAdded"),
        "Key 'recentlyAdded' not found in the directory: {:#?}",
        m
    );
    assert!(
        m.contains_key("onDeck"),
        "Key 'onDeck'' not found in the directory: {:#?}",
        m
    );
}
