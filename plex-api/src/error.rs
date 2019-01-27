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
impl From<serde_xml_rs::Error> for PlexApiError {
    fn from(e: serde_xml_rs::Error) -> Self {
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

impl From<serde_json::Error> for PlexApiError {
    fn from(e: serde_json::Error) -> Self {
        println!("{:#?}", e);
        Self {}
    }
}
