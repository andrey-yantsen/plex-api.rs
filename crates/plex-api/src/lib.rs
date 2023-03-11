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
pub use http_client::{HttpClient, HttpClientBuilder};
pub use media_container::{
    devices::Feature,
    preferences::Value as SettingValue,
    server::{
        library::{AudioCodec, ContainerFormat, Decision, Protocol, SubtitleCodec, VideoCodec},
        Feature as ServerFeature, MappingState as ServerMappingState,
    },
};
pub use myplex::{device, pin::PinManager, sharing, MyPlex, MyPlexBuilder};
pub use player::Player;
pub use server::{
    library::{
        Artist, Collection, Episode, Item, Library, MediaItem, MetadataItem, Movie, MusicAlbum,
        Photo, PhotoAlbum, PhotoAlbumItem, Playlist, Season, Show, Track, Video,
    },
    transcode::{
        ArtTranscodeOptions, AudioSetting, Constraint, Limitation, MusicTranscodeOptions,
        TranscodeSession, TranscodeStatus, VideoSetting, VideoTranscodeOptions,
    },
    Server,
};

pub type Result<T = (), E = error::Error> = std::result::Result<T, E>;
