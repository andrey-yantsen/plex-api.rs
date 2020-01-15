use crate::{
    my_plex::{MyPlexAccount, MyPlexApiErrorResponse},
    InternalHttpApi, PlexApiError,
};
use reqwest::StatusCode;
use serde_json;

const WEBHOOKS_URL: &str = "api/v2/user/webhooks";

#[derive(Debug, Deserialize)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
struct Webhook {
    url: String,
}

impl MyPlexAccount {
    /// Returns a list of URLs for currently registered WebHooks.
    pub async fn get_webhooks(&self) -> crate::Result<Vec<String>> {
        let response = self.get(WEBHOOKS_URL).await?;
        if response.status() == StatusCode::OK {
            let hooks: Vec<Webhook> = serde_json::from_str(response.text().await?.as_str())?;
            let mut ret: Vec<String> = Vec::new();
            for hook in hooks {
                ret.push(hook.url)
            }
            Ok(ret)
        } else {
            let err: MyPlexApiErrorResponse = response.json().await?;
            Err(core::convert::From::from(err))
        }
    }

    /// Sets a list of WebHooks to provided URLs list.
    pub async fn set_webhooks(&self, webhooks: &[&str]) -> crate::Result<()> {
        let params = if webhooks.is_empty() {
            vec![("urls", "")]
        } else {
            webhooks.iter().map(|&url| ("urls[]", url)).collect()
        };

        let response = self.post_form(WEBHOOKS_URL, &params).await?;
        if response.status() == StatusCode::CREATED {
            Ok(())
        } else {
            let err: MyPlexApiErrorResponse = response.json().await?;
            Err(core::convert::From::from(err))
        }
    }

    /// Add an URL to webhooks list.
    pub async fn add_webhook(&self, webhook: &str) -> crate::Result<()> {
        let webhooks = self.get_webhooks().await?;
        let mut webhooks: Vec<&str> = webhooks.iter().map(|s| &**s).collect();
        webhooks.push(webhook);
        self.set_webhooks(&webhooks).await
    }

    /// Deletes provided URL from webhooks list.
    pub async fn del_webhook(&self, webhook: &str) -> crate::Result<()> {
        let webhooks: Vec<String> = self.get_webhooks().await?;
        let original_len = webhooks.len();
        let new_webhooks: Vec<&str> = webhooks
            .iter()
            .filter(|&value| *value != *webhook)
            .map(|s| &**s)
            .collect();
        if original_len == new_webhooks.len() {
            Err(PlexApiError::WebhookNotFound {
                url: String::from(webhook),
            })
        } else {
            self.set_webhooks(&new_webhooks).await
        }
    }
}
