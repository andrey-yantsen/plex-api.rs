use std::marker::PhantomData;

use enum_dispatch::enum_dispatch;

use crate::{
    media_container::{
        server::library::{
            LibraryType, Metadata, MetadataMediaContainer, MetadataType, ServerLibrary,
        },
        MediaContainerWrapper,
    },
    HttpClient, Result,
};

pub trait FromMetadata {
    /// Creates an item given the http configuration and item metadata. No
    /// validation is performed that the metadata is correct.
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self;
}

/// Implements MetadataItem for the given struct which must only contain `client`
/// and `metadata` fields.
macro_rules! derive_from_metadata {
    ($typ:ident) => {
        impl FromMetadata for $typ {
            fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
                Self { client, metadata }
            }
        }
    };
}

/// Functionality shared across different items types in the Plex library.
#[enum_dispatch]
pub trait MetadataItem {
    /// Returns the Plex metadata for this item.
    fn metadata(&self) -> &Metadata;
    /// Returns the http client for this item.
    fn client(&self) -> &HttpClient;

    /// Returns the title of this item.
    fn title(&self) -> &str {
        &self.metadata().title
    }
}

/// Implements MetadataItem for the given struct which must contain `client`
/// and `metadata` fields.
macro_rules! derive_metadata_item {
    ($typ:ident) => {
        impl MetadataItem for $typ {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                &self.client
            }
        }
    };
    ($typ:ident<$gen:ident>) => {
        impl<$gen> MetadataItem for $typ<$gen> {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                &self.client
            }
        }
    };
}

/// Retrieves a list of metadata items given the lookup key.
async fn metadata_items<T>(client: &HttpClient, key: &str) -> Result<Vec<T>>
where
    T: FromMetadata,
{
    let wrapper: MediaContainerWrapper<MetadataMediaContainer> = client.get(key).json().await?;

    let media = wrapper
        .media_container
        .metadata
        .into_iter()
        .map(|metadata| T::from_metadata(client.clone(), metadata))
        .collect();
    Ok(media)
}

/// Attempts to retrieve the parent of this item.
async fn parent<T, P>(item: &T, client: &HttpClient) -> Result<Option<P>>
where
    T: MetadataItem,
    P: FromMetadata,
{
    if let Some(ref parent_key) = item.metadata().parent.parent_key {
        Ok(metadata_items(client, parent_key).await?.into_iter().next())
    } else {
        Ok(None)
    }
}

/// Retrieves the metadata items from a pivot from a library.
async fn pivot_items<M>(
    client: &HttpClient,
    directory: &ServerLibrary,
    context: &str,
) -> Result<Vec<M>>
where
    M: FromMetadata,
{
    if let Some(pivot) = directory.pivots.iter().find(|p| p.context == context) {
        metadata_items(client, &pivot.key).await
    } else {
        Ok(Vec::new())
    }
}

/// A video that can be included in a video playlist.
#[enum_dispatch(MetadataItem)]
pub enum Video {
    Movie,
    Episode,
}

impl FromMetadata for Video {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        if let Some(MetadataType::Episode) = metadata.metadata_type {
            Episode::from_metadata(client, metadata).into()
        } else {
            Movie::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Playlist<M> {
    _items: PhantomData<M>,
    client: HttpClient,
    metadata: Metadata,
}

impl<M> FromMetadata for Playlist<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Playlist<M>);

impl<M> Playlist<M>
where
    M: FromMetadata,
{
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(&self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Collection<M> {
    _items: PhantomData<M>,
    client: HttpClient,
    metadata: Metadata,
}

impl<M> FromMetadata for Collection<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Collection<M>);

impl<M> Collection<M>
where
    M: FromMetadata,
{
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(&self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Movie {
    #[allow(dead_code)]
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Movie);
derive_metadata_item!(Movie);

#[derive(Debug, Clone)]
pub struct Show {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Show);
derive_metadata_item!(Show);

impl Show {
    /// Retrieves all of the seasons of this show.
    pub async fn seasons(&self) -> Result<Vec<Season>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves all of the episodes in all seasons of this show.
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        let path = format!("/library/metadata/{}/allLeaves", self.metadata.rating_key);
        metadata_items(&self.client, &path).await
    }
}

#[derive(Debug, Clone)]
pub struct Season {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Season);
derive_metadata_item!(Season);

impl Season {
    pub fn season_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves all of the episodes in this season.
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    // Retrieves the show that this season is from.
    pub async fn show(&self) -> Result<Option<Show>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Episode {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Episode);
derive_metadata_item!(Episode);

impl Episode {
    // Returns the number of this season within the show.
    pub fn season_number(&self) -> Option<u32> {
        self.metadata.parent.parent_index
    }

    // Returns the number of this episode within the season.
    pub fn episode_number(&self) -> Option<u32> {
        self.metadata.index
    }

    // Retrieves the season that this episode is from.
    pub async fn season(&self) -> Result<Option<Season>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Artist {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Artist);
derive_metadata_item!(Artist);

impl Artist {
    /// Retrieves all of the albums by this artist.
    pub async fn albums(&self) -> Result<Vec<MusicAlbum>> {
        metadata_items(&self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct MusicAlbum {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(MusicAlbum);
derive_metadata_item!(MusicAlbum);

impl MusicAlbum {
    /// Retrieves all of the tracks in this album.
    pub async fn tracks(&self) -> Result<Vec<Track>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves the artist for this album.
    pub async fn artist(&self) -> Result<Option<Artist>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Track);
derive_metadata_item!(Track);

impl Track {
    // Returns the number of this track within the album.
    pub fn track_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves the album for this track.
    pub async fn album(&self) -> Result<Option<MusicAlbum>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Photo {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Photo);
derive_metadata_item!(Photo);

impl Photo {
    /// Retrieves the album that this photo is in.
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoAlbum {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(PhotoAlbum);
derive_metadata_item!(PhotoAlbum);

impl PhotoAlbum {
    /// Retrieves all of the albums and photos in this album.
    pub async fn contents(&self) -> Result<Vec<PhotoAlbumItem>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves the album that this album is in.
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, &self.client).await
    }
}

#[enum_dispatch(MetadataItem)]
pub enum PhotoAlbumItem {
    PhotoAlbum,
    Photo,
}

impl FromMetadata for PhotoAlbumItem {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        // This isn't a great test but there doesn't seem to be much better.
        if metadata.key.ends_with("/children") {
            PhotoAlbum::from_metadata(client, metadata).into()
        } else {
            Photo::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl MovieLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the movies in this library.
    pub async fn movies(&self) -> Result<Vec<Movie>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    pub async fn collections(&self) -> Result<Vec<Collection<Movie>>> {
        pivot_items(&self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing movies from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct TVLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl TVLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the shows in this library.
    pub async fn shows(&self) -> Result<Vec<Show>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    pub async fn collections(&self) -> Result<Vec<Collection<Show>>> {
        pivot_items(&self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing episodes from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct MusicLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl MusicLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the artists in this library.
    pub async fn artists(&self) -> Result<Vec<Artist>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing tracks from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Track>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl PhotoLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the albums in this library.
    pub async fn albums(&self) -> Result<Vec<PhotoAlbum>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing photos from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Photo>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub enum Library {
    Movie(MovieLibrary),
    TV(TVLibrary),
    Music(MusicLibrary),
    Video(MovieLibrary),
    Photo(PhotoLibrary),
}

impl Library {
    pub(super) fn new(client: HttpClient, directory: ServerLibrary) -> Self {
        match directory.library_type {
            LibraryType::Movie => {
                if directory.subtype.as_deref() == Some("clip") {
                    Library::Video(MovieLibrary { client, directory })
                } else {
                    Library::Movie(MovieLibrary { client, directory })
                }
            }
            LibraryType::Show => Library::TV(TVLibrary { client, directory }),
            LibraryType::Artist => Library::Music(MusicLibrary { client, directory }),
            LibraryType::Photo => Library::Photo(PhotoLibrary { client, directory }),
            #[cfg(not(feature = "tests_deny_unknown_fields"))]
            LibraryType::Unknown => panic!("Unknown library type"),
        }
    }

    fn directory(&self) -> &ServerLibrary {
        match self {
            Self::Movie(l) => &l.directory,
            Self::TV(l) => &l.directory,
            Self::Music(l) => &l.directory,
            Self::Video(l) => &l.directory,
            Self::Photo(l) => &l.directory,
        }
    }

    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory().title
    }
}
