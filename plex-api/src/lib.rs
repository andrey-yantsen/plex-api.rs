extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate quick_xml;
extern crate serde_aux;
extern crate serde_json;
extern crate serde_with;

use reqwest::header::HeaderMap;

use async_trait::async_trait;

pub use self::config::{
    X_PLEX_CLIENT_IDENTIFIER, X_PLEX_DEVICE, X_PLEX_DEVICE_NAME, X_PLEX_PLATFORM,
    X_PLEX_PLATFORM_VERSION, X_PLEX_PRODUCT, X_PLEX_PROVIDES, X_PLEX_SYNC_VERSION, X_PLEX_VERSION,
};
pub use self::error::*;
pub use self::http::{clear_headers_cache, set_http_client};
pub use self::media_container::*;
pub use self::my_plex::*;
pub use self::server::*;
use url::Url;

mod config;
mod error;
mod http;
mod library;
mod media_container;
mod my_plex;
pub mod prelude;
mod serde_helpers;
mod server;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, crate::error::PlexApiError>;

#[async_trait]
trait CanBeDeleted {
    async fn delete(&mut self) -> Result<reqwest::Response>;
}

trait HasDeleteUrl {
    fn get_delete_url(&self) -> Option<String>;
}

#[async_trait]
impl<T: HasDeleteUrl + CanMakeRequests + Send + Sync> CanBeDeleted for T {
    /// Remove current object from your account / server.
    async fn delete(&mut self) -> Result<reqwest::Response> {
        let url = self.get_delete_url();

        if let Some(url) = url {
            self.prepare_query(&url, reqwest::Method::DELETE)?
                .send()
                .await
                .map_err(From::from)
        } else {
            Err(PlexApiError::DeleteUrlIsNotProvided)
        }
    }
}

trait HasPlexHeaders {
    fn headers(&self) -> Result<HeaderMap>;
}

trait HasBaseUrl {
    fn get_base_url(&self) -> &str;
}

trait CanMakeRequests {
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

impl AsStr for Url {
    fn as_str(&self) -> &str {
        self.as_str()
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
                        let mut request_url = String::from(self.get_base_url());
                        if !request_url.ends_with('/') {
                            request_url.push('/');
                        }
                        request_url.push_str(s);
                        url::Url::parse(&request_url).map_err(core::convert::From::from)
                    }
                    _ => Err(PlexApiError::from(e)),
                },
            }
        };

        // in tests we can't use Client Singleton due to connections caching across different tokio
        // runtimes, and this leads to unexpected errors in requests

        #[cfg(not(test))]
        let client = http::get_http_client()?;

        #[cfg(test)]
        let client = reqwest::Client::builder()
            .timeout(core::time::Duration::from_secs(30))
            .connect_timeout(core::time::Duration::from_secs(5))
            .build()
            .expect("HTTP_CLIENT init");

        Ok(client
            .request(method, request_url?)
            .headers(self.headers()?))
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
