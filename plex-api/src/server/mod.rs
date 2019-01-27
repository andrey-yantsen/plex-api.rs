use crate::media_container::MediaContainer;

mod connect;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Server {
    info: MediaContainer,
    url: String,
}
