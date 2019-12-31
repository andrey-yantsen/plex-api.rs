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
use std::sync::{LockResult, PoisonError, RwLockReadGuard, RwLockWriteGuard};

use reqwest::{header::HeaderMap, Client};
use uuid::Uuid;

use async_trait::async_trait;
use uname::uname;

pub use self::error::*;
pub use self::media_container::*;
pub use self::my_plex::*;
pub use self::server::*;

mod error;
mod media_container;
mod my_plex;
mod serde_helpers;
mod server;
#[cfg(test)]
mod tests;

const PROJECT: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const X_PLEX_SYNC_VERSION: &str = "2";

lazy_static! {
    /// `X-Plex-Provides` header value.
    ///
    /// Should be one or more of `controller`, `server`, `sync-target`, `player`.
    pub static ref X_PLEX_PROVIDES: RwLock<&'static str> = RwLock::new("controller");

    /// `X-Plex-Platform` header value.
    ///
    /// Platform name, e.g. iOS, macOS, etc.
    pub static ref X_PLEX_PLATFORM: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Platform-Version` header value.
    ///
    /// OS version, e.g. 4.3.1
    pub static ref X_PLEX_PLATFORM_VERSION: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Product` header value.
    ///
    /// Application name, e.g. Laika, Plex Media Server, Media Link.
    pub static ref X_PLEX_PRODUCT: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Version` header value.
    ///
    /// Application version, e.g. 10.6.7.
    pub static ref X_PLEX_VERSION: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Device` header value.
    ///
    /// Device name and model number, e.g. iPhone3,2, Motorola XOOM™, LG5200TV.
    pub static ref X_PLEX_DEVICE: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Device-Name` header value.
    ///
    /// Primary name for the device, e.g. "Plex Web (Chrome)".
    pub static ref X_PLEX_DEVICE_NAME: RwLock<&'static str> = RwLock::new("");

    /// `X-Plex-Client-Identifier` header value.
    ///
    /// UUID, serial number, or other number unique per device. Random value would be generated
    /// if not set. Can be set by providing ENV-value `X_PLEX_CLIENT_IDENTIFIER`.
    ///
    /// **N.B.** Should be unique across all your devices.
    pub static ref X_PLEX_CLIENT_IDENTIFIER: RwLock<&'static str> = RwLock::new("");
    static ref HTTP_CLIENT: RwLock<Client> = {
        #[cfg(all(test, not(any(feature = "test_connect_authenticated", feature = "test_connect_anonymous"))))]
        {
            panic!("Client shouldn't be requested in this testing mode")
        }
        #[cfg(any(not(test), feature = "test_connect_authenticated", feature = "test_connect_anonymous"))]
        {
            use std::time::Duration;
            RwLock::new(
                Client::builder()
                    .timeout(Duration::from_secs(5))
                    .build()
                    .expect("HTTP_CLIENT init")
            )
        }
    };
}

/// A method to set custom HTTP-client, e.g. to change request timeout or to set a proxy.
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
///     set_http_client(Client::builder()
///                         .timeout(Duration::from_secs(1))
///                         .proxy(Proxy::http("http://example.com").expect("Proxy failed"))
///                         .build()
///                         .expect("Build failed")
///     ).expect("Mutex poisoned");
/// ```
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
        client_identifier = match client_id {
            Ok(id) => id,
            Err(_) => {
                warn!(target: "plex-api", "Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
                let random_uuid = Uuid::new_v4();
                random_uuid.to_string()
            }
        };
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

pub type Result<T> = std::result::Result<T, crate::error::PlexApiError>;

#[async_trait]
pub trait CanBeDeleted {
    async fn delete(&mut self) -> Result<reqwest::Response>;
}

pub trait HasDeleteUrl {
    fn get_delete_url(&self) -> Option<String>;
}

#[async_trait]
impl<T: HasDeleteUrl + CanMakeRequests + Send + Sync> CanBeDeleted for T {
    /// Remove current object from your account.
    async fn delete(&mut self) -> Result<reqwest::Response> {
        let url = self.get_delete_url();

        if let Some(url) = url {
            self.prepare_query(&url, reqwest::Method::DELETE)?
                .send()
                .await
                .map_err(From::from)
        } else {
            // TODO: Better error
            Err(crate::error::PlexApiError {})
        }
    }
}

pub trait HasPlexHeaders {
    fn headers(&self) -> HeaderMap;
}

pub trait HasBaseUrl {
    fn get_base_url(&self) -> String;
}

pub trait CanMakeRequests {
    fn prepare_query<P: reqwest::IntoUrl + AsStr>(
        &self,
        url: P,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder>;
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for &str {
    fn as_str(&self) -> &str {
        self
    }
}

impl AsStr for &String {
    fn as_str(&self) -> &str {
        self
    }
}

impl<T: HasPlexHeaders + HasBaseUrl> CanMakeRequests for T {
    fn prepare_query<P: reqwest::IntoUrl + AsStr>(
        &self,
        url: P,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder> {
        let request_url = {
            let s = url.as_str();
            match url::Url::parse(s) {
                Ok(u) => Ok(u),
                Err(e) => match e {
                    url::ParseError::RelativeUrlWithoutBase => {
                        let mut request_url = self.get_base_url();
                        if !request_url.ends_with('/') {
                            request_url.push('/');
                        }
                        request_url.push_str(s);
                        url::Url::parse(&request_url).map_err(PlexApiError::from)
                    }
                    _ => Err(PlexApiError::from(e)),
                },
            }
        };

        Ok(get_http_client()?
            .request(method, request_url?)
            .headers(self.headers()))
    }
}

#[async_trait]
trait InternalHttpApi {
    async fn get<U: reqwest::IntoUrl + AsStr + Send>(
        &self,
        url: U,
    ) -> crate::Result<reqwest::Response>;
    async fn post_form<U: reqwest::IntoUrl + AsStr + Send, T: serde::Serialize + ?Sized + Sync>(
        &self,
        url: U,
        params: &T,
    ) -> crate::Result<reqwest::Response>;
    async fn put_form<U: reqwest::IntoUrl + AsStr + Send, T: serde::Serialize + ?Sized + Sync>(
        &self,
        url: U,
        params: &T,
    ) -> crate::Result<reqwest::Response>;
}

#[async_trait]
impl<T: CanMakeRequests + Sync> InternalHttpApi for T {
    async fn get<U: reqwest::IntoUrl + AsStr + Send>(&self, url: U) -> Result<reqwest::Response> {
        self.prepare_query(url, reqwest::Method::GET)?
            .send()
            .await
            .map_err(From::from)
    }

    async fn post_form<U: reqwest::IntoUrl + AsStr + Send, P: serde::Serialize + ?Sized + Sync>(
        &self,
        url: U,
        params: &P,
    ) -> Result<reqwest::Response> {
        self.prepare_query(url, reqwest::Method::POST)?
            .form(params)
            .send()
            .await
            .map_err(From::from)
    }

    async fn put_form<U: reqwest::IntoUrl + AsStr + Send, P: serde::Serialize + ?Sized + Sync>(
        &self,
        url: U,
        params: &P,
    ) -> Result<reqwest::Response> {
        self.prepare_query(url, reqwest::Method::POST)?
            .form(params)
            .send()
            .await
            .map_err(From::from)
    }
}
