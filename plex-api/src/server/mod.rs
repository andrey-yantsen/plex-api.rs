use crate::media_container::MediaContainer;

mod connect;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "strict_deserialize", serde(deny_unknown_fields))]
pub struct Server {
    info: MediaContainer,
    url: String,
}

#[derive(Debug, Clone)]
pub struct ServerError {}

type Result<T> = std::result::Result<T, ServerError>;

// TODO: Implement error conversion
impl From<reqwest::Error> for ServerError {
    fn from(e: reqwest::Error) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, reqwest::Client>>> for ServerError {
    fn from(e: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, reqwest::Client>>) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}

// TODO: Implement error conversion
impl From<serde_xml_rs::Error> for ServerError {
    fn from(e: serde_xml_rs::Error) -> Self {
        eprintln!("{:#?}", e);
        Self {}
    }
}
