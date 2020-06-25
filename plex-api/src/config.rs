use reqwest::Client;
use std::sync::RwLock;

pub const PROJECT: Option<&'static str> = option_env!("CARGO_PKG_NAME");
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const SUPPORTED_X_PLEX_SYNC_VERSION: &str = "2";

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

    /// `X-Plex-Sync-Version` header value.
    ///
    /// The lib supports only "2", but you can set any value at your own risk.
    pub static ref X_PLEX_SYNC_VERSION: RwLock<&'static str> = RwLock::new(SUPPORTED_X_PLEX_SYNC_VERSION);

    /// `X-Plex-Client-Identifier` header value.
    ///
    /// UUID, serial number, or other number unique per device. Random value would be generated
    /// if not set. Can be set by providing ENV-value `X_PLEX_CLIENT_IDENTIFIER`.
    ///
    /// **N.B.** Should be unique for each of your devices.
    pub static ref X_PLEX_CLIENT_IDENTIFIER: RwLock<String> = RwLock::new(String::new());
    pub static ref HTTP_CLIENT: RwLock<Client> = {
        #[cfg(all(test, not(any(feature = "test_connect_authenticated", feature = "test_connect_anonymous"))))]
        {
            panic!("Client shouldn't be requested in this testing mode")
        }
        #[cfg(any(not(test), feature = "test_connect_authenticated", feature = "test_connect_anonymous"))]
        {
            use std::time::Duration;
            RwLock::new(
                Client::builder()
                    .timeout(Duration::from_secs(30))
                    .connect_timeout(Duration::from_secs(5))
                    .build()
                    .expect("HTTP_CLIENT init")
            )
        }
    };
}
