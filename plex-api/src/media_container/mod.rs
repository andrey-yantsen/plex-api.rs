mod device;
mod library;
mod server;
mod settings;
mod user;

pub use self::device::*;
pub use self::library::*;
pub use self::server::*;
pub use self::settings::*;
pub use self::user::*;

use crate::serde_helpers::option_int_from_string;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    #[serde(deserialize_with = "option_int_from_string", default)]
    size: Option<u32>,
    #[serde(deserialize_with = "option_int_from_string", default)]
    total_size: Option<u32>,
    public_address: Option<String>,
    friendly_name: Option<String>,
    identifier: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct Directory {
    count: u16,
    key: String,
    title: String,
}
