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
    preferences::{Setting, SettingEnumValue, Value as SettingValue},
    server::{
        library::{
            AudioCodec, ContainerFormat, Decision, Field, GrandParentMetadata, Guid, Location,
            Media as MediaMetadata, Metadata, MetadataType, ParentMetadata, Part as PartMetadata,
            PlaylistType, Protocol, Rating, Role, SubtitleCodec, Tag, VideoCodec,
        },
        Feature as ServerFeature, MappingState as ServerMappingState,
    },
};
pub use myplex::{
    account::RestrictionProfile, device, pin::PinManager, sharing, MyPlex, MyPlexBuilder,
};
pub use player::Player;
pub use server::{
    library::{
        Artist, Collection, Episode, Item, Library, Media, MediaItem, MediaItemWithTranscoding,
        MetadataItem, Movie, MusicAlbum, Part, Photo, PhotoAlbum, PhotoAlbumItem, Playlist, Season,
        Show, Track, Video,
    },
    prefs::Preferences,
    transcode::{
        ArtTranscodeOptions, AudioSetting, Constraint, Limitation, MusicTranscodeOptions,
        TranscodeSession, TranscodeSessionStats, TranscodeStatus, VideoSetting,
        VideoTranscodeOptions,
    },
    Server,
};

pub type Result<T = (), E = error::Error> = std::result::Result<T, E>;
