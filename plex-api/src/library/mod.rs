use crate::{DirectoryMediaContainer, Result};
use crate::{
    InternalHttpApi, LibraryMediaContainer, LibraryMediaContainerOuter, PlexApiError, Server,
};

#[derive(Debug)]
pub struct Library<'a> {
    server: &'a Server,
    info: LibraryMediaContainer,
}

const LIBRARY_SECTIONS_URL: &str = "library/sections";
const LIBRARY_RECENTLY_ADDED_URL: &str = "library/recentlyAdded";
const LIBRARY_ON_DECK_URL: &str = "library/onDeck";

impl<'a> Library<'a> {
    pub(crate) async fn sections(server: &'a Server) -> Result<Library<'a>> {
        Self::load(server, LIBRARY_SECTIONS_URL).await
    }

    pub(crate) async fn recently_added(server: &'a Server) -> Result<Library<'a>> {
        Self::load(server, LIBRARY_RECENTLY_ADDED_URL).await
    }

    pub(crate) async fn on_deck(server: &'a Server) -> Result<Library<'a>> {
        Self::load(server, LIBRARY_ON_DECK_URL).await
    }

    async fn load(server: &'a Server, url: &str) -> Result<Library<'a>> {
        let response = server.get(url).await?;
        if response.status() == reqwest::StatusCode::OK {
            let mc =
                LibraryMediaContainer::from(response.json::<LibraryMediaContainerOuter>().await?);
            Ok(Library { server, info: mc })
        } else {
            Err(PlexApiError::UnexpectedApiResponse(response.text().await?))
        }
    }

    pub const fn get_directory(&self) -> &Option<Vec<DirectoryMediaContainer>> {
        &self.info.directory
    }
}
