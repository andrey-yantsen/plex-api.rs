use std::marker::PhantomData;

use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde_json::from_str;

use crate::{
    media_container::{
        server::library::{
            LibraryType, Metadata, MetadataMediaContainer, MetadataType, ServerLibrary,
        },
        MediaContainerWrapper,
    },
    HttpClient, Result,
};

/// Functionality shared across different items types in the Plex library.
pub trait MetadataItem {
    /// Creates an item given the http configuration and item metadata. No
    /// validation is performed that the metadata is correct.
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self;
    /// Returns the Plex metadata for this item.
    fn metadata(&self) -> &Metadata;
    /// Returns the http client for this item.
    fn client(&self) -> &HttpClient;

    /// Returns the title of this item.
    fn title(&self) -> &str {
        &self.metadata().title
    }
}

/// Retrieves a list of metadata items given the lookup key.
async fn metadata_items<T>(client: &HttpClient, key: &str) -> Result<Vec<T>>
where
    T: MetadataItem,
{
    let mut response = client
        .get(key)
        .header("Accept", "application/json")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let text = response.text().await?;
            let wrapper: MediaContainerWrapper<MetadataMediaContainer> = match from_str(&text) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{text}");
                    return Err(e.into());
                }
            };
            let media = wrapper
                .media_container
                .metadata
                .into_iter()
                .map(|metadata| T::from_metadata(client.clone(), metadata))
                .collect();
            Ok(media)
        }
        _ => Err(crate::Error::from_response(response).await),
    }
}

/// Attempts to retrieve the parent of this item.
async fn parent<T, P>(item: &T, client: &HttpClient) -> Result<Option<P>>
where
    T: MetadataItem,
    P: MetadataItem,
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
    M: MetadataItem,
{
    if let Some(pivot) = directory.pivots.iter().find(|p| p.context == context) {
        metadata_items(client, &pivot.key).await
    } else {
        Ok(Vec::new())
    }
}

/// A video that can be included in a video playlist.
pub enum Video {
    Movie(Movie),
    Episode(Episode),
}

impl MetadataItem for Video {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        if let Some(MetadataType::Episode) = metadata.metadata_type {
            Video::Episode(Episode::from_metadata(client, metadata))
        } else {
            Video::Movie(Movie::from_metadata(client, metadata))
        }
    }

    fn metadata(&self) -> &Metadata {
        match self {
            Video::Movie(m) => m.metadata(),
            Video::Episode(m) => m.metadata(),
        }
    }

    fn client(&self) -> &HttpClient {
        match self {
            Video::Movie(m) => m.client(),
            Video::Episode(m) => m.client(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Playlist<M> {
    _items: PhantomData<M>,
    client: HttpClient,
    metadata: Metadata,
}

impl<M> MetadataItem for Playlist<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

impl<M> Playlist<M>
where
    M: MetadataItem,
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

impl<M> MetadataItem for Collection<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

impl<M> Collection<M>
where
    M: MetadataItem,
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

impl MetadataItem for Movie {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Movie { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

#[derive(Debug, Clone)]
pub struct Show {
    client: HttpClient,
    metadata: Metadata,
}

impl MetadataItem for Show {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Show { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for Season {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Season { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for Episode {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Episode { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for Artist {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Artist { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for MusicAlbum {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        MusicAlbum { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for Track {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Track { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for Photo {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Photo { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

impl MetadataItem for PhotoAlbum {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        PhotoAlbum { client, metadata }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn client(&self) -> &HttpClient {
        &self.client
    }
}

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

#[derive(Debug, Clone)]
pub enum PhotoAlbumItem {
    Album(PhotoAlbum),
    Photo(Photo),
}

impl MetadataItem for PhotoAlbumItem {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        // This isn't a great test but there doesn't seem to be much better.
        if metadata.key.ends_with("/children") {
            PhotoAlbumItem::Album(PhotoAlbum::from_metadata(client, metadata))
        } else {
            PhotoAlbumItem::Photo(Photo::from_metadata(client, metadata))
        }
    }

    fn metadata(&self) -> &Metadata {
        match self {
            Self::Album(a) => a.metadata(),
            Self::Photo(p) => p.metadata(),
        }
    }

    fn client(&self) -> &HttpClient {
        match self {
            Self::Album(a) => a.client(),
            Self::Photo(p) => p.client(),
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
