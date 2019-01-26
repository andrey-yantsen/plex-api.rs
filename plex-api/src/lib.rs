extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_with;

use std::env;
use std::result;
use std::sync::RwLock;
use std::time::Duration;

use reqwest::{header::HeaderMap, Client};
use uname::uname;
use uuid::Uuid;

use std::sync::{LockResult, PoisonError, RwLockReadGuard, RwLockWriteGuard};

mod media_container;
mod my_plex;
mod serde_helpers;
mod server;
#[cfg(test)]
mod tests;

pub use self::my_plex::*;
pub use self::server::*;

const PROJECT: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const X_PLEX_SYNC_VERSION: &str = "2";

lazy_static! {
    pub static ref X_PLEX_PROVIDES: RwLock<&'static str> = RwLock::new("controller");
    pub static ref X_PLEX_PLATFORM: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_PLATFORM_VERSION: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_PRODUCT: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_VERSION: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_DEVICE: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_DEVICE_NAME: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_CLIENT_IDENTIFIER: RwLock<&'static str> = RwLock::new("");
    static ref HTTP_CLIENT: RwLock<Client> = RwLock::new(
        Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("HTTP_CLIENT init")
    );
}

pub fn set_http_client(
    c: Client,
) -> result::Result<(), PoisonError<RwLockWriteGuard<'static, Client>>> {
    let mut client = HTTP_CLIENT.write()?;
    *client = c;
    Ok(())
}

fn get_http_client() -> LockResult<RwLockReadGuard<'static, Client>> {
    HTTP_CLIENT.read()
}

fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let i = uname().unwrap();

    let provides = *X_PLEX_PROVIDES.read().unwrap();
    headers.insert("X-Plex-Provides", provides.parse().unwrap());

    let mut product = *X_PLEX_PRODUCT.read().unwrap();
    if product == "" {
        product = PROJECT.unwrap_or("plex-api");
    }

    headers.insert("X-Plex-Product", product.parse().unwrap());

    let mut version = *X_PLEX_VERSION.read().unwrap();
    if version == "" {
        version = VERSION.unwrap_or("unknown");
    }

    headers.insert("X-Plex-Version", version.parse().unwrap());
    headers.insert("X-Plex-Sync-Version", X_PLEX_SYNC_VERSION.parse().unwrap());

    let mut platform = *X_PLEX_PLATFORM.read().unwrap();
    if platform == "" {
        platform = &i.sysname;
    }

    headers.insert("X-Plex-Platform", platform.parse().unwrap());

    let mut platform_version = *X_PLEX_PLATFORM_VERSION.read().unwrap();
    if platform_version == "" {
        platform_version = &i.release;
    }

    headers.insert("X-Plex-Platform-Version", platform_version.parse().unwrap());

    let mut client_identifier: String = String::from(*X_PLEX_CLIENT_IDENTIFIER.read().unwrap());
    if client_identifier == "" {
        let client_id = env::var("X_PLEX_CLIENT_IDENTIFIER");
        if client_id.is_ok() {
            client_identifier = client_id.unwrap().clone();
        } else {
            warn!(target: "plex-api", "Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
            let random_uuid = Uuid::new_v4();
            client_identifier = random_uuid.to_string().clone();
        }
    }

    headers.insert(
        "X-Plex-Client-Identifier",
        client_identifier.parse().unwrap(),
    );

    let mut device = *X_PLEX_DEVICE.read().unwrap();
    if device == "" {
        device = platform
    }

    headers.insert("X-Plex-Device", device.parse().unwrap());

    let mut device_name = *X_PLEX_DEVICE_NAME.read().unwrap();
    if device_name == "" {
        device_name = &i.nodename;
    }

    headers.insert("X-Plex-Device-Name", device_name.parse().unwrap());

    headers
}
