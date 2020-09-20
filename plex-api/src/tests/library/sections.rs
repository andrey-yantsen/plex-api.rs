test_case_online_all!(_directory_populated);

async fn _directory_populated(srv: crate::Server) {
    let library = srv.get_sections().await;
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
        m.contains_key("1"),
        "Key 1 (Movies) not found in the directory: {:#?}",
        m
    );
    assert!(
        m.contains_key("5"),
        "Key 5 (Music) not found in the directory: {:#?}",
        m
    );
    assert!(
        m.contains_key("3"),
        "Key 3 (Photos) not found in the directory: {:#?}",
        m
    );
    assert!(
        m.contains_key("2"),
        "Key 2 (TV-Shows) not found in the directory: {:#?}",
        m
    );
}
