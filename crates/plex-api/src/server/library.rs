use std::marker::PhantomData;

use enum_dispatch::enum_dispatch;

use crate::{
    media_container::{
        server::library::{
            LibraryType, Metadata, MetadataMediaContainer, MetadataType, PlaylistType,
            ServerLibrary,
        },
        MediaContainerWrapper,
    },
    HttpClient, Result,
};

pub trait FromMetadata<'a> {
    /// Creates an item given the http configuration and item metadata. No
    /// validation is performed that the metadata is correct.
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self;
}

/// Implements MetadataItem for the given struct which must only contain `client`
/// and `metadata` fields.
macro_rules! derive_from_metadata {
    ($typ:ident) => {
        impl<'a> FromMetadata<'a> for $typ<'a> {
            fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
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

    /// Returns the rating key for this item.
    ///
    /// This can be used to re-retrieve the item at a later time through the
    /// Server::item_by_id function.
    fn rating_key(&self) -> u32 {
        self.metadata().rating_key
    }

    /// Returns the title of this item.
    fn title(&self) -> &str {
        &self.metadata().title
    }
}

/// Implements MetadataItem for the given struct which must contain `client`
/// and `metadata` fields.
macro_rules! derive_metadata_item {
    ($typ:ident) => {
        impl<'a> MetadataItem for $typ<'a> {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                self.client
            }
        }
    };
    ($typ:ident<$gen:ident>) => {
        impl<'a, $gen> MetadataItem for $typ<'a, $gen> {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                self.client
            }
        }
    };
}

/// Retrieves a list of metadata items given the lookup key.
pub(crate) async fn metadata_items<'a, T>(client: &'a HttpClient, key: &str) -> Result<Vec<T>>
where
    T: FromMetadata<'a>,
{
    let wrapper: MediaContainerWrapper<MetadataMediaContainer> = client.get(key).json().await?;

    let media = wrapper
        .media_container
        .metadata
        .into_iter()
        .map(|metadata| T::from_metadata(client, metadata))
        .collect();
    Ok(media)
}

/// Attempts to retrieve the parent of this item.
async fn parent<'a, T, P>(item: &T, client: &'a HttpClient) -> Result<Option<P>>
where
    T: MetadataItem,
    P: FromMetadata<'a>,
{
    if let Some(ref parent_key) = item.metadata().parent.parent_key {
        Ok(metadata_items(client, parent_key).await?.into_iter().next())
    } else {
        Ok(None)
    }
}

/// Retrieves the metadata items from a pivot from a library.
async fn pivot_items<'a, M>(
    client: &'a HttpClient,
    directory: &ServerLibrary,
    context: &str,
) -> Result<Vec<M>>
where
    M: FromMetadata<'a>,
{
    if let Some(pivot) = directory.pivots.iter().find(|p| p.context == context) {
        metadata_items(client, &pivot.key).await
    } else {
        Ok(Vec::new())
    }
}

/// A video that can be included in a video playlist.
#[enum_dispatch(MetadataItem)]
#[derive(Debug, Clone)]
pub enum Video<'a> {
    Movie(Movie<'a>),
    Episode(Episode<'a>),
}

impl<'a> FromMetadata<'a> for Video<'a> {
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
        if let Some(MetadataType::Episode) = metadata.metadata_type {
            Episode::from_metadata(client, metadata).into()
        } else {
            Movie::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Playlist<'a, M> {
    _items: PhantomData<M>,
    client: &'a HttpClient,
    metadata: Metadata,
}

impl<'a, M> FromMetadata<'a> for Playlist<'a, M> {
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Playlist<M>);

impl<'a, M> Playlist<'a, M>
where
    M: FromMetadata<'a>,
{
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Collection<'a, M> {
    _items: PhantomData<M>,
    client: &'a HttpClient,
    metadata: Metadata,
}

impl<'a, M> FromMetadata<'a> for Collection<'a, M> {
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Collection<M>);

impl<'a, M> Collection<'a, M>
where
    M: FromMetadata<'a>,
{
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Movie<'a> {
    #[allow(dead_code)]
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Movie);
derive_metadata_item!(Movie);

#[derive(Debug, Clone)]
pub struct Show<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Show);
derive_metadata_item!(Show);

impl<'a> Show<'a> {
    /// Retrieves all of the seasons of this show.
    pub async fn seasons(&self) -> Result<Vec<Season>> {
        metadata_items(self.client, &self.metadata.key).await
    }

    /// Retrieves all of the episodes in all seasons of this show.
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        let path = format!("/library/metadata/{}/allLeaves", self.metadata.rating_key);
        metadata_items(self.client, &path).await
    }
}

#[derive(Debug, Clone)]
pub struct Season<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Season);
derive_metadata_item!(Season);

impl<'a> Season<'a> {
    pub fn season_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves all of the episodes in this season.
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        metadata_items(self.client, &self.metadata.key).await
    }

    // Retrieves the show that this season is from.
    pub async fn show(&self) -> Result<Option<Show>> {
        parent(self, self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Episode<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Episode);
derive_metadata_item!(Episode);

impl<'a> Episode<'a> {
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
        parent(self, self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Artist);
derive_metadata_item!(Artist);

impl<'a> Artist<'a> {
    /// Retrieves all of the albums by this artist.
    pub async fn albums(&self) -> Result<Vec<MusicAlbum>> {
        metadata_items(self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct MusicAlbum<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(MusicAlbum);
derive_metadata_item!(MusicAlbum);

impl<'a> MusicAlbum<'a> {
    /// Retrieves all of the tracks in this album.
    pub async fn tracks(&self) -> Result<Vec<Track>> {
        metadata_items(self.client, &self.metadata.key).await
    }

    /// Retrieves the artist for this album.
    pub async fn artist(&self) -> Result<Option<Artist>> {
        parent(self, self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Track<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Track);
derive_metadata_item!(Track);

impl<'a> Track<'a> {
    // Returns the number of this track within the album.
    pub fn track_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves the album for this track.
    pub async fn album(&self) -> Result<Option<MusicAlbum>> {
        parent(self, self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Photo<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Photo);
derive_metadata_item!(Photo);

impl<'a> Photo<'a> {
    /// Retrieves the album that this photo is in.
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoAlbum<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(PhotoAlbum);
derive_metadata_item!(PhotoAlbum);

impl<'a> PhotoAlbum<'a> {
    /// Retrieves all of the albums and photos in this album.
    pub async fn contents(&self) -> Result<Vec<PhotoAlbumItem<'a>>> {
        metadata_items(self.client, &self.metadata.key).await
    }

    /// Retrieves the album that this album is in.
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, self.client).await
    }
}

#[enum_dispatch(MetadataItem)]
pub enum PhotoAlbumItem<'a> {
    PhotoAlbum(PhotoAlbum<'a>),
    Photo(Photo<'a>),
}

impl<'a> FromMetadata<'a> for PhotoAlbumItem<'a> {
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
        // This isn't a great test but there doesn't seem to be much better.
        if metadata.key.ends_with("/children") {
            PhotoAlbum::from_metadata(client, metadata).into()
        } else {
            Photo::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownItem<'a> {
    client: &'a HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(UnknownItem);
derive_metadata_item!(UnknownItem);

#[enum_dispatch(MetadataItem)]
#[derive(Debug, Clone)]
pub enum Item<'a> {
    Movie(Movie<'a>),
    Episode(Episode<'a>),
    Photo(Photo<'a>),
    Show(Show<'a>),
    Artist(Artist<'a>),
    MusicAlbum(MusicAlbum<'a>),
    Season(Season<'a>),
    Track(Track<'a>),
    MovieCollection(Collection<'a, Movie<'a>>),
    ShowCollection(Collection<'a, Show<'a>>),
    VideoPlaylist(Playlist<'a, Video<'a>>),
    PhotoPlaylist(Playlist<'a, Photo<'a>>),
    MusicPlaylist(Playlist<'a, Track<'a>>),
    UnknownItem(UnknownItem<'a>),
}

impl<'a> FromMetadata<'a> for Item<'a> {
    fn from_metadata(client: &'a HttpClient, metadata: Metadata) -> Self {
        if let Some(ref item_type) = metadata.metadata_type {
            match item_type {
                MetadataType::Movie => Movie::from_metadata(client, metadata).into(),
                MetadataType::Episode => Episode::from_metadata(client, metadata).into(),
                MetadataType::Photo => Photo::from_metadata(client, metadata).into(),
                MetadataType::Show => Show::from_metadata(client, metadata).into(),
                MetadataType::Artist => Artist::from_metadata(client, metadata).into(),
                MetadataType::MusicAlbum => MusicAlbum::from_metadata(client, metadata).into(),
                MetadataType::Season => Season::from_metadata(client, metadata).into(),
                MetadataType::Track => Track::from_metadata(client, metadata).into(),
                MetadataType::Collection => match metadata.subtype {
                    Some(MetadataType::Movie) => {
                        Collection::<Movie>::from_metadata(client, metadata).into()
                    }
                    Some(MetadataType::Show) => {
                        Collection::<Show>::from_metadata(client, metadata).into()
                    }
                    _ => UnknownItem::from_metadata(client, metadata).into(),
                },
                MetadataType::Playlist => match metadata.playlist_type {
                    Some(PlaylistType::Video) => {
                        Playlist::<Video>::from_metadata(client, metadata).into()
                    }
                    Some(PlaylistType::Audio) => {
                        Playlist::<Track>::from_metadata(client, metadata).into()
                    }
                    Some(PlaylistType::Photo) => {
                        Playlist::<Photo>::from_metadata(client, metadata).into()
                    }
                    _ => UnknownItem::from_metadata(client, metadata).into(),
                },
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
                _ => UnknownItem::from_metadata(client, metadata).into(),
            }
        } else {
            UnknownItem::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieLibrary<'a> {
    client: &'a HttpClient,
    directory: ServerLibrary,
}

impl<'a> MovieLibrary<'a> {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the movies in this library.
    pub async fn movies(&self) -> Result<Vec<Movie>> {
        pivot_items(self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    pub async fn collections(&self) -> Result<Vec<Collection<Movie>>> {
        pivot_items(self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing movies from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct TVLibrary<'a> {
    client: &'a HttpClient,
    directory: ServerLibrary,
}

impl<'a> TVLibrary<'a> {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the shows in this library.
    pub async fn shows(&self) -> Result<Vec<Show>> {
        pivot_items(self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    pub async fn collections(&self) -> Result<Vec<Collection<Show>>> {
        pivot_items(self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing episodes from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct MusicLibrary<'a> {
    client: &'a HttpClient,
    directory: ServerLibrary,
}

impl<'a> MusicLibrary<'a> {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the artists in this library.
    pub async fn artists(&self) -> Result<Vec<Artist>> {
        pivot_items(self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing tracks from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Track>>> {
        pivot_items(self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoLibrary<'a> {
    client: &'a HttpClient,
    directory: ServerLibrary,
}

impl<'a> PhotoLibrary<'a> {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the albums in this library.
    pub async fn albums(&self) -> Result<Vec<PhotoAlbum>> {
        pivot_items(self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing photos from this library.
    pub async fn playlists(&self) -> Result<Vec<Playlist<Photo>>> {
        pivot_items(self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub enum Library<'a> {
    Movie(MovieLibrary<'a>),
    TV(TVLibrary<'a>),
    Music(MusicLibrary<'a>),
    Video(MovieLibrary<'a>),
    Photo(PhotoLibrary<'a>),
}

impl<'a> Library<'a> {
    pub(super) fn new(client: &'a HttpClient, directory: ServerLibrary) -> Self {
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
