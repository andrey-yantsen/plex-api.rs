use crate::{url::MYPLEX_DEFAULT_API_URL, Result};
use core::convert::TryFrom;
use http::{uri::PathAndQuery, Uri};
use isahc::{
    config::{Configurable, RedirectPolicy},
    http::request::Builder,
    AsyncBody, HttpClient as IsahcHttpClient, Request as HttpRequest, Response as HttpResponse,
};
use std::time::Duration;
use uuid::Uuid;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_CONNECTIONT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub api_url: Uri,

    pub http_client: IsahcHttpClient,

    /// `X-Plex-Provides` header value. Comma-separated list.
    ///
    /// Should be one or more of `controller`, `server`, `sync-target`, `player`.
    pub x_plex_provides: String,

    /// `X-Plex-Platform` header value.
    ///
    /// Platform name, e.g. iOS, macOS, etc.
    pub x_plex_platform: String,

    /// `X-Plex-Platform-Version` header value.
    ///
    /// OS version, e.g. 4.3.1
    pub x_plex_platform_version: String,

    /// `X-Plex-Product` header value.
    ///
    /// Application name, e.g. Laika, Plex Media Server, Media Link.
    pub x_plex_product: String,

    /// `X-Plex-Version` header value.
    ///
    /// Application version, e.g. 10.6.7.
    pub x_plex_version: String,

    /// `X-Plex-Device` header value.
    ///
    /// Device name and model number, e.g. iPhone3,2, Motorola XOOM™, LG5200TV.
    pub x_plex_device: String,

    /// `X-Plex-Device-Name` header value.
    ///
    /// Primary name for the device, e.g. "Plex Web (Chrome)".
    pub x_plex_device_name: String,

    /// `X-Plex-Client-Identifier` header value.
    ///
    /// UUID, serial number, or other number unique per device.
    ///
    /// **N.B.** Should be unique for each of your devices.
    pub x_plex_client_identifier: String,

    /// `X-Plex-Token` header value.
    ///
    /// Auth token for Plex.
    x_plex_token: String,

    /// `X-Plex-Sync-Version` header value.
    ///
    /// Not sure what are the valid values, but at the time of writing Plex Web sends `2` here.
    pub x_plex_sync_version: String,
}

impl HttpClient {
    fn prepare_request(&self) -> Builder {
        self.prepare_request_min()
            .header("X-Plex-Provides", &self.x_plex_provides)
            .header("X-Plex-Platform", &self.x_plex_platform)
            .header("X-Plex-Platform-Version", &self.x_plex_platform_version)
            .header("X-Plex-Product", &self.x_plex_product)
            .header("X-Plex-Version", &self.x_plex_version)
            .header("X-Plex-Device", &self.x_plex_device)
            .header("X-Plex-Device-Name", &self.x_plex_device_name)
            .header("X-Plex-Sync-Version", &self.x_plex_sync_version)
    }

    fn prepare_request_min(&self) -> Builder {
        let mut request = HttpRequest::builder()
            .header("X-Plex-Client-Identifier", &self.x_plex_client_identifier);

        if !self.x_plex_token.is_empty() {
            request = request.header("X-Plex-Token", &self.x_plex_token);
        }

        request
    }

    pub fn is_authenticated(&self) -> bool {
        !self.x_plex_token.is_empty()
    }

    pub fn post<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("POST"),
        }
    }

    /// Does the same as HttpClient::post(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn postm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("POST"),
        }
    }

    pub fn get<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("GET"),
        }
    }

    /// Does the same as HttpClient::get(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn getm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("GET"),
        }
    }

    pub fn put<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("PUT"),
        }
    }

    /// Does the same as HttpClient::put(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn putm<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request_min().method("PUT"),
        }
    }

    pub fn delete<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("DELETE"),
        }
    }

    /// Does the same as HttpClient::delete(), but appends only bare minimum
    /// headers: `X-Plex-Client-Identifier` and `X-Plex-Token`.
    pub fn deletem<T>(&self, path: T) -> RequestBuilder<'_, T>
    where
        PathAndQuery: TryFrom<T>,
        <PathAndQuery as TryFrom<T>>::Error: Into<http::Error>,
    {
        RequestBuilder {
            http_client: &self.http_client,
            base_url: self.api_url.clone(),
            path_and_query: path,
            request_builder: self.prepare_request().method("DELETE"),
        }
    }

    /// Set the client's authentication token.
    pub fn set_x_plex_token(self, x_plex_token: String) -> Self {
        Self {
            x_plex_token,
            ..self
        }
    }

    /// Get a reference to the client's authentication token.
    pub fn x_plex_token(&self) -> &str {
        self.x_plex_token.as_ref()
    }
}

pub struct RequestBuilder<'a, P>
where
    PathAndQuery: TryFrom<P>,
    <PathAndQuery as TryFrom<P>>::Error: Into<http::Error>,
{
    http_client: &'a IsahcHttpClient,
    base_url: Uri,
    path_and_query: P,
    request_builder: Builder,
}

impl<'a, P> RequestBuilder<'a, P>
where
    PathAndQuery: TryFrom<P>,
    <PathAndQuery as TryFrom<P>>::Error: Into<http::Error>,
{
    pub fn body<B>(self, body: B) -> Result<Request<'a, B>>
    where
        B: Into<AsyncBody> + std::fmt::Debug,
    {
        let path_and_query = PathAndQuery::try_from(self.path_and_query).map_err(Into::into)?;
        let mut uri_parts = self.base_url.into_parts();
        uri_parts.path_and_query = Some(path_and_query);
        let uri = Uri::from_parts(uri_parts).map_err(Into::<http::Error>::into)?;

        Ok(Request {
            http_client: self.http_client,
            request: self.request_builder.uri(uri).body(body)?,
        })
    }

    pub fn form(self, params: &[(&str, &str)]) -> Result<Request<'a, String>> {
        let body = serde_urlencoded::to_string(params)?;
        self.header("Content-type", "application/x-www-form-urlencoded")
            .header("Content-Length", body.len().to_string())
            .body(body)
    }

    pub fn header<K, V>(self, key: K, value: V) -> RequestBuilder<'a, P>
    where
        http::header::HeaderName: TryFrom<K>,
        <http::header::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::header::HeaderValue: TryFrom<V>,
        <http::header::HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        Self {
            http_client: self.http_client,
            base_url: self.base_url,
            path_and_query: self.path_and_query,
            request_builder: self.request_builder.header(key, value),
        }
    }

    pub async fn send(self) -> Result<HttpResponse<AsyncBody>> {
        self.body(())?.send().await
    }
}

pub struct Request<'a, T> {
    http_client: &'a IsahcHttpClient,
    request: HttpRequest<T>,
}

impl<'a, T> Request<'a, T>
where
    T: Into<AsyncBody>,
{
    pub async fn send(self) -> Result<HttpResponse<AsyncBody>> {
        Ok(self.http_client.send_async(self.request).await?)
    }
}

pub struct HttpClientBuilder {
    client: Result<HttpClient>,
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        let sys_platform = sys_info::os_type().unwrap_or_else(|_| "unknown".to_string());
        let sys_version = sys_info::os_release().unwrap_or_else(|_| "unknown".to_string());
        let sys_hostname = sys_info::hostname().unwrap_or_else(|_| "unknown".to_string());

        let random_uuid = Uuid::new_v4();

        let client = HttpClient {
            api_url: Uri::from_static(MYPLEX_DEFAULT_API_URL),
            http_client: IsahcHttpClient::builder()
                .timeout(DEFAULT_TIMEOUT)
                .connect_timeout(DEFAULT_CONNECTIONT_TIMEOUT)
                .redirect_policy(RedirectPolicy::None)
                .build()
                .expect("failed to create default http client"),
            x_plex_provides: String::from("controller"),
            x_plex_product: option_env!("CARGO_PKG_NAME")
                .unwrap_or("plex-api")
                .to_string(),
            x_plex_platform: sys_platform.clone(),
            x_plex_platform_version: sys_version,
            x_plex_version: option_env!("CARGO_PKG_VERSION")
                .unwrap_or("unknown")
                .to_string(),
            x_plex_device: sys_platform,
            x_plex_device_name: sys_hostname,
            x_plex_client_identifier: random_uuid.to_string(),
            x_plex_sync_version: String::from("2"),
            x_plex_token: String::new(),
        };

        Self { client: Ok(client) }
    }
}

impl HttpClientBuilder {
    pub fn build(self) -> Result<HttpClient> {
        self.client
    }

    pub fn set_http_client(self, http_client: IsahcHttpClient) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.http_client = http_client;
                client
            }),
        }
    }

    pub fn from(client: HttpClient) -> Self {
        Self { client: Ok(client) }
    }

    pub fn new<U>(api_url: U) -> Self
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Self::default().set_api_url(api_url)
    }

    pub fn set_api_url<U>(self, api_url: U) -> Self
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Self {
            client: self.client.and_then(move |mut client| {
                client.api_url = Uri::try_from(api_url).map_err(Into::into)?;
                Ok(client)
            }),
        }
    }

    pub fn set_x_plex_token(self, token: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_token = token;
                client
            }),
        }
    }

    pub fn set_x_plex_client_identifier(self, client_identifier: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_client_identifier = client_identifier;
                client
            }),
        }
    }

    pub fn set_x_plex_provides(self, x_plex_provides: Vec<String>) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_provides = x_plex_provides.join(",");
                client
            }),
        }
    }

    pub fn set_x_plex_platform(self, platform: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_platform = platform;
                client
            }),
        }
    }

    pub fn set_x_plex_platform_version(self, platform_version: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_platform_version = platform_version;
                client
            }),
        }
    }

    pub fn set_x_plex_product(self, product: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_product = product;
                client
            }),
        }
    }

    pub fn set_x_plex_version(self, version: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_version = version;
                client
            }),
        }
    }

    pub fn set_x_plex_device(self, device: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_device = device;
                client
            }),
        }
    }

    pub fn set_x_plex_device_name(self, device_name: String) -> Self {
        Self {
            client: self.client.map(move |mut client| {
                client.x_plex_device_name = device_name;
                client
            }),
        }
    }
}
