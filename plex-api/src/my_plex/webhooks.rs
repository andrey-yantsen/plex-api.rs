use crate::my_plex::MyPlexApiErrorResponse;
use crate::my_plex::{MyPlexAccount, MyPlexError, Result};
use reqwest::StatusCode;
use serde_json;

const WEBHOOKS_URL: &str = "https://plex.tv/api/v2/user/webhooks";

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Webhook {
    url: String,
}

impl MyPlexAccount {
    pub fn get_webhooks(&self) -> Result<Vec<String>> {
        let mut response = self.get(WEBHOOKS_URL)?;
        if response.status() == StatusCode::OK {
            let hooks: Vec<Webhook> = serde_json::from_str(response.text()?.as_str())?;
            let mut ret: Vec<String> = Vec::new();
            for hook in hooks {
                ret.push(hook.url)
            }
            Ok(ret)
        } else {
            let err: MyPlexApiErrorResponse = response.json()?;
            Err(MyPlexError::from(err))
        }
    }

    pub fn set_webhooks(&self, webhooks: &[&str]) -> Result<()> {
        let params = if webhooks.is_empty() {
            vec![("urls", "")]
        } else {
            webhooks.iter().map(|&url| ("urls[]", url)).collect()
        };

        let mut response = self.post_form(WEBHOOKS_URL, &params)?;
        if response.status() == StatusCode::CREATED {
            Ok(())
        } else {
            let err: MyPlexApiErrorResponse = response.json()?;
            Err(MyPlexError::from(err))
        }
    }

    pub fn add_webhook(&self, webhook: &str) -> Result<()> {
        let webhooks = self.get_webhooks()?;
        let mut webhooks: Vec<&str> = webhooks.iter().map(|s| &**s).collect();
        webhooks.push(webhook);
        self.set_webhooks(&webhooks)
    }

    pub fn del_webhook(&self, webhook: &str) -> Result<()> {
        let webhooks: Vec<String> = self.get_webhooks()?;
        let original_len = webhooks.len();
        let new_webhooks: Vec<&str> = webhooks
            .iter()
            .filter(|&value| *value != *webhook)
            .map(|s| &**s)
            .collect();
        if original_len == new_webhooks.len() {
            Err(MyPlexError {})
        } else {
            self.set_webhooks(&new_webhooks)
        }
    }
}

impl From<serde_json::Error> for MyPlexError {
    fn from(e: serde_json::Error) -> Self {
        println!("{:#?}", e);
        Self {}
    }
}
