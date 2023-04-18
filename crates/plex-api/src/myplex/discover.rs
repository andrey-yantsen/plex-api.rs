use crate::{
    library::{metadata_items, Item},
    media_container::server::{MediaProvider, MediaProviderWrapper},
    url::MYPLEX_DISCOVER_API_BASE_URL,
    Error, HttpClient, HttpClientBuilder, Result,
};

#[derive(Debug, Clone)]
pub struct Discover {
    pub provider: MediaProvider,
    client: HttpClient,
}

impl Discover {
    pub async fn new<C: Into<HttpClient>>(client: C) -> Result<Self> {
        let client = HttpClientBuilder::from(client.into())
            .set_api_url(MYPLEX_DISCOVER_API_BASE_URL)
            .build()?;

        let w: MediaProviderWrapper = client.get("/").json().await?;
        Ok(Self {
            provider: w.media_provider,
            client,
        })
    }

    /// Allows retrieving media items using their rating key.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn item_by_id(&self, rating_key: &str) -> Result<Item> {
        let path = format!("/library/metadata/{rating_key}?includeConcerts=1&includeExtras=1&includePreferences=1&includeReviews=1&includeOnDeck=1&includeChapters=1&includeStations=1&includeExternalMedia=1&asyncAugmentMetadata=1&asyncCheckFiles=1&asyncRefreshAnalysis=1&asyncRefreshLocalMediaAgent=1");

        match metadata_items(&self.client, &path).await {
            Ok(items) => items.into_iter().next().ok_or(Error::ItemNotFound),
            Err(Error::UnexpectedApiResponse {
                status_code,
                content,
            }) => {
                // A 404 error indicates the item does not exist.
                if status_code == 404 {
                    Err(Error::ItemNotFound)
                } else {
                    Err(Error::UnexpectedApiResponse {
                        status_code,
                        content,
                    })
                }
            }
            Err(err) => Err(err),
        }
    }
}
