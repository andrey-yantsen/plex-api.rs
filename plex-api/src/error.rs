#[derive(Debug, Clone)]
pub struct PlexApiError {}

// TODO: Implement error conversion
impl From<reqwest::Error> for PlexApiError {
    fn from(e: reqwest::Error) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<quick_xml::de::DeError> for PlexApiError {
    fn from(e: quick_xml::de::DeError) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, reqwest::Client>>>
    for PlexApiError
{
    fn from(e: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, reqwest::Client>>) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<serde_json::Error> for PlexApiError {
    fn from(e: serde_json::Error) -> Self {
        println!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<url::ParseError> for PlexApiError {
    fn from(e: url::ParseError) -> Self {
        println!("{:#?}", e);
        Self {}
    }
}
