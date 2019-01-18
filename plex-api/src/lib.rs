#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_with;

use reqwest::header::HeaderMap;
use uname::uname;
use uuid::Uuid;

pub use self::my_plex::*;

mod media_container;
mod my_plex;

#[cfg(test)]
mod tests {
    use crate::{
        base_headers, X_PLEX_CLIENT_IDENTIFIER, X_PLEX_DEVICE, X_PLEX_DEVICE_NAME, X_PLEX_PLATFORM,
        X_PLEX_PLATFORM_VERSION, X_PLEX_PRODUCT, X_PLEX_PROVIDES, X_PLEX_SYNC_VERSION,
        X_PLEX_VERSION,
    };

    #[test]
    fn base_headers_contains_required_headers() {
        let headers = base_headers();
        assert_eq!(true, headers.contains_key("x-plex-provides"));
        assert_eq!(true, headers.contains_key("x-plex-product"));
        assert_eq!(true, headers.contains_key("x-plex-version"));
        assert_eq!(true, headers.contains_key("x-plex-sync-version"));
        assert_eq!(true, headers.contains_key("x-plex-platform"));
        assert_eq!(true, headers.contains_key("x-plex-platform-version"));
        assert_eq!(true, headers.contains_key("x-plex-client-identifier"));
        assert_eq!(true, headers.contains_key("x-plex-device"));
        assert_eq!(true, headers.contains_key("x-plex-device-name"));
    }

    #[test]
    fn base_headers_use_provided_values() {
        unsafe {
            X_PLEX_PROVIDES = "plex_provides";
            X_PLEX_PLATFORM = "plex_platform";
            X_PLEX_PLATFORM_VERSION = "plex_platform_version";
            X_PLEX_PRODUCT = "plex_product";
            X_PLEX_VERSION = "plex_version";
            X_PLEX_DEVICE = "plex_device";
            X_PLEX_DEVICE_NAME = "plex_device_name";
            X_PLEX_CLIENT_IDENTIFIER = "plex_client_identifier";

            let headers = base_headers();
            assert_eq!(X_PLEX_PROVIDES, headers.get("x-plex-provides").unwrap());
            assert_eq!(X_PLEX_PRODUCT, headers.get("x-plex-product").unwrap());
            assert_eq!(X_PLEX_VERSION, headers.get("x-plex-version").unwrap());
            assert_eq!(
                X_PLEX_SYNC_VERSION,
                headers.get("x-plex-sync-version").unwrap()
            );
            assert_eq!(X_PLEX_PLATFORM, headers.get("x-plex-platform").unwrap());
            assert_eq!(
                X_PLEX_PLATFORM_VERSION,
                headers.get("x-plex-platform-version").unwrap()
            );
            assert_eq!(
                X_PLEX_CLIENT_IDENTIFIER,
                headers.get("x-plex-client-identifier").unwrap()
            );
            assert_eq!(X_PLEX_DEVICE, headers.get("x-plex-device").unwrap());
            assert_eq!(
                X_PLEX_DEVICE_NAME,
                headers.get("x-plex-device-name").unwrap()
            );
        }
    }
}

const PROJECT: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const X_PLEX_SYNC_VERSION: &str = "2";

pub static mut X_PLEX_PROVIDES: &'static str = "controller";
pub static mut X_PLEX_PLATFORM: &'static str = "";
pub static mut X_PLEX_PLATFORM_VERSION: &'static str = "";
pub static mut X_PLEX_PRODUCT: &'static str = "";
pub static mut X_PLEX_VERSION: &'static str = "";
pub static mut X_PLEX_DEVICE: &'static str = "";
pub static mut X_PLEX_DEVICE_NAME: &'static str = "";
pub static mut X_PLEX_CLIENT_IDENTIFIER: &'static str = "";

fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let i = uname().unwrap();

    unsafe {
        headers.insert("X-Plex-Provides", X_PLEX_PROVIDES.parse().unwrap());

        if X_PLEX_PRODUCT == "" {
            X_PLEX_PRODUCT = PROJECT.unwrap_or("plex-api.rs");
        }

        headers.insert("X-Plex-Product", X_PLEX_PRODUCT.parse().unwrap());

        if X_PLEX_VERSION == "" {
            X_PLEX_VERSION = VERSION.unwrap_or("unknown");
        }

        headers.insert("X-Plex-Version", X_PLEX_VERSION.parse().unwrap());
        headers.insert("X-Plex-Sync-Version", X_PLEX_SYNC_VERSION.parse().unwrap());

        if X_PLEX_PLATFORM == "" {
            X_PLEX_PLATFORM = Box::leak(i.sysname.into_boxed_str());
        }

        headers.insert("X-Plex-Platform", X_PLEX_PLATFORM.parse().unwrap());

        if X_PLEX_PLATFORM_VERSION == "" {
            X_PLEX_PLATFORM_VERSION = Box::leak(i.release.into_boxed_str());
        }

        headers.insert(
            "X-Plex-Platform-Version",
            X_PLEX_PLATFORM_VERSION.parse().unwrap(),
        );

        if X_PLEX_CLIENT_IDENTIFIER == "" {
            let client_id = env::var("X_PLEX_CLIENT_IDENTIFIER");
            if client_id.is_ok() {
                X_PLEX_CLIENT_IDENTIFIER = Box::leak(
                    String::from(client_id.unwrap())
                        .to_string()
                        .into_boxed_str(),
                );
            } else {
                warn!(target: "plex-api", "Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
                let random_uuid = Uuid::new_v4();
                X_PLEX_CLIENT_IDENTIFIER = Box::leak(random_uuid.to_string().into_boxed_str());
            }
        }

        headers.insert(
            "X-Plex-Client-Identifier",
            X_PLEX_CLIENT_IDENTIFIER.parse().unwrap(),
        );

        if X_PLEX_DEVICE == "" {
            X_PLEX_DEVICE = X_PLEX_PLATFORM.clone()
        }

        headers.insert("X-Plex-Device", X_PLEX_DEVICE.parse().unwrap());

        if X_PLEX_DEVICE_NAME == "" {
            X_PLEX_DEVICE_NAME = Box::leak(i.nodename.into_boxed_str());
        }

        headers.insert("X-Plex-Device-Name", X_PLEX_DEVICE_NAME.parse().unwrap());
    }

    headers
}

use serde::de::{self, Deserialize, Deserializer, Unexpected};
use std::env;
use std::result;

fn bool_from_int<'de, D>(deserializer: D) -> result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn option_bool_from_int<'de, D>(deserializer: D) -> result::Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match bool_from_int(deserializer) {
        Ok(v) => Ok(Option::from(v)),
        Err(e) => Err(e),
    }
}
