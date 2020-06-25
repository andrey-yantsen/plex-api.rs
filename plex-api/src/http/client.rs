use crate::config;
use crate::Result;
use reqwest::{header::HeaderMap, Client};
use std::sync::{LockResult, RwLock, RwLockReadGuard};
use uname::uname;
use uuid::Uuid;

/// Sets custom HTTP-client, e.g. to change request timeout or to set a proxy.
///
/// Error would be returned if [`RwLock`] had been poisoned.
///
/// [`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
///
/// # Examples
///
/// ```
/// use plex_api::set_http_client;
/// use reqwest::{Client, Proxy};
/// use std::time::Duration;
///
/// set_http_client(Client::builder()
///                     .timeout(Duration::from_secs(1))
///                     .proxy(Proxy::http("http://example.com").expect("Proxy failed"))
///                     .build()
///                     .expect("Build failed")
/// ).expect("Mutex poisoned");
/// ```
pub fn set_http_client(c: Client) -> Result<()> {
    let mut client = config::HTTP_CLIENT.write()?;
    *client = c;
    Ok(())
}

pub fn get_http_client() -> LockResult<RwLockReadGuard<'static, Client>> {
    // override http-client in tests, to prevent using cached connections across different tokio runtimes
    #[cfg(test)]
    {
        debug!("Resetting HTTP_CLIENT to default value to prevent connections caching");
        set_http_client(
            Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Build failed"),
        )
        .expect("Failed to reset http-client");
    }
    config::HTTP_CLIENT.read()
}

lazy_static! {
    static ref HEADERS_CACHE: RwLock<HeaderMap> = RwLock::new(HeaderMap::new());
}

/// Resets cached headers
///
/// Useful when you need to change headers at the runtime.
pub fn clear_headers_cache() -> Result<()> {
    let mut cache = HEADERS_CACHE.write()?;
    *cache = HeaderMap::new();
    Ok(())
}

pub fn base_headers() -> Result<HeaderMap> {
    {
        let cache = HEADERS_CACHE.read()?;
        if !cache.is_empty() {
            return Ok(cache.clone());
        }
    }

    let mut headers = HeaderMap::new();
    let i = uname()?;

    let provides = *config::X_PLEX_PROVIDES.read()?;
    headers.insert("X-Plex-Provides", provides.parse()?);

    let mut product = *config::X_PLEX_PRODUCT.read()?;
    if product == "" {
        product = config::PROJECT.unwrap_or("plex-api");
    }

    headers.insert("X-Plex-Product", product.parse()?);

    let mut version = *config::X_PLEX_VERSION.read()?;
    if version == "" {
        version = config::VERSION.unwrap_or("unknown");
    }

    headers.insert("X-Plex-Version", version.parse()?);
    headers.insert(
        "X-Plex-Sync-Version",
        config::X_PLEX_SYNC_VERSION.read()?.parse()?,
    );

    let mut platform = *config::X_PLEX_PLATFORM.read()?;
    if platform == "" {
        platform = &i.sysname;
    }

    headers.insert("X-Plex-Platform", platform.parse()?);

    let mut platform_version = *config::X_PLEX_PLATFORM_VERSION.read()?;
    if platform_version == "" {
        platform_version = &i.release;
    }

    headers.insert("X-Plex-Platform-Version", platform_version.parse()?);

    let mut client_identifier: String = String::from(&*config::X_PLEX_CLIENT_IDENTIFIER.read()?);
    if client_identifier == "" {
        warn!("Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
        let random_uuid = Uuid::new_v4();
        client_identifier = random_uuid.to_string();
    }

    headers.insert("X-Plex-Client-Identifier", client_identifier.parse()?);

    let mut device = *config::X_PLEX_DEVICE.read()?;
    if device == "" {
        device = platform
    }

    headers.insert("X-Plex-Device", device.parse()?);

    let mut device_name = *config::X_PLEX_DEVICE_NAME.read()?;
    if device_name == "" {
        device_name = &i.nodename;
    }

    headers.insert("X-Plex-Device-Name", device_name.parse()?);

    let mut cache = HEADERS_CACHE.write()?;
    *cache = headers.clone();

    Ok(headers)
}
