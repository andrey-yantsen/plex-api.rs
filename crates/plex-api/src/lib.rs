//! TODO
//!
mod error;
mod http_client;
pub(crate) mod media_container;
mod myplex;
mod player;
mod server;
pub mod url;
pub mod webhook;

pub use error::Error;
pub use http_client::HttpClient;
pub use http_client::HttpClientBuilder;
pub use media_container::devices::Feature;
pub use media_container::preferences::Value as SettingValue;
pub use media_container::server::library::{
    AudioCodec, ContainerFormat, Decision, Protocol, SubtitleCodec, VideoCodec,
};
pub use media_container::server::Feature as ServerFeature;
pub use myplex::sharing;
pub use myplex::{device, pin::PinManager, MyPlex, MyPlexBuilder};
pub use player::Player;
pub use server::library::{
    Artist, Collection, Episode, Item, Library, MediaItem, MetadataItem, Movie, MusicAlbum, Photo,
    PhotoAlbum, PhotoAlbumItem, Playlist, Season, Show, Track, Video,
};
pub use server::transcode::{
    ArtTranscodeOptions, AudioSetting, Constraint, Limitation, MusicTranscodeOptions,
    TranscodeSession, TranscodeStatus, VideoSetting, VideoTranscodeOptions,
};
pub use server::Server;

pub type Result<T = (), E = error::Error> = std::result::Result<T, E>;
