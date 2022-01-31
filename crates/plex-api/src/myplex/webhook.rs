use crate::http_client::HttpClient;
use crate::url::MYPLEX_WEBHOOKS_PATH;
use crate::{Error, Result};
use core::convert::TryFrom;
use http::{StatusCode, Uri};
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use std::sync::Arc;

pub struct WebhookManager {
    webhooks: Vec<Webhook>,

    client: Arc<HttpClient>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Webhook {
    #[serde(with = "http_serde::uri")]
    url: Uri,
}

impl Webhook {
    /// Get a reference to the webhook's url.
    pub fn url(&self) -> &Uri {
        &self.url
    }
}

impl WebhookManager {
    pub async fn new(client: Arc<HttpClient>) -> Result<Self> {
        let mut ret = Self {
            webhooks: vec![],
            client,
        };
        ret.refresh().await?;
        Ok(ret)
    }

    pub async fn refresh(&mut self) -> Result {
        let mut response = self.client.get(MYPLEX_WEBHOOKS_PATH).send().await?;

        match response.status() {
            StatusCode::OK => {
                self.webhooks = response.json().await?;
                Ok(())
            }
            _ => Err(Error::from_response(response).await),
        }
    }

    pub fn webhooks(&self) -> &[Webhook] {
        &self.webhooks
    }

    pub async fn set(&mut self, webhooks: Vec<Webhook>) -> Result {
        let webhook_urls = webhooks
            .iter()
            .map(|webhook| webhook.url.to_string())
            .collect::<Vec<_>>();

        let params = if webhooks.is_empty() {
            vec![("urls", "")]
        } else {
            webhook_urls
                .iter()
                .map(|webhook_url| ("urls[]", webhook_url.as_str()))
                .collect()
        };

        let response = self
            .client
            .post(MYPLEX_WEBHOOKS_PATH)
            .form(&params)?
            .send()
            .await?;
        if response.status() == StatusCode::CREATED {
            self.webhooks = webhooks;
            Ok(())
        } else {
            Err(Error::from_response(response).await)
        }
    }

    pub async fn add<U>(&mut self, url: U) -> Result
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let mut webhooks = self.webhooks.clone();
        webhooks.push(Webhook {
            url: Uri::try_from(url).map_err(Into::into)?,
        });
        self.set(webhooks).await
    }

    pub async fn delete<U>(&mut self, url: U) -> Result
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let original_len = self.webhooks.len();
        let webhook_url = Uri::try_from(url).map_err(Into::into)?;
        let new_webhooks: Vec<Webhook> = self
            .webhooks
            .clone()
            .into_iter()
            .filter(|value| value.url != webhook_url)
            .collect();
        if original_len == new_webhooks.len() {
            Err(Error::WebhookNotFound(format!("{}", webhook_url)))
        } else {
            self.set(new_webhooks).await
        }
    }
}
