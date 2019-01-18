use crate::my_plex::MyPlexApiErrorResponse;
use crate::my_plex::{MyPlex, MyPlexError, Result};
use reqwest::StatusCode;
use serde_json;

const WEBHOOKS_URL: &str = "https://plex.tv/api/v2/user/webhooks";

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Webhook {
    url: String,
}

impl MyPlex {
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

    pub fn set_webhooks(&self, webhooks: Vec<String>) -> Result<()> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if webhooks.len() > 0 {
            for url in &webhooks {
                params.push(("urls[]", url.as_str()));
            }
        } else {
            params.push(("urls", ""));
        }
        let mut response = self.post_form(WEBHOOKS_URL, &params)?;
        if response.status() == StatusCode::CREATED {
            Ok(())
        } else {
            let err: MyPlexApiErrorResponse = response.json()?;
            Err(MyPlexError::from(err))
        }
    }

    pub fn add_webhook(&self, webhook: String) -> Result<()> {
        let mut webhooks = self.get_webhooks()?;
        webhooks.push(webhook);
        self.set_webhooks(webhooks)
    }

    pub fn del_webhook(&self, webhook: String) -> Result<()> {
        let mut webhooks = self.get_webhooks()?;
        let mut del_idx = webhooks.len() + 1;

        for (idx, value) in webhooks.iter().enumerate() {
            if *value == webhook {
                del_idx = idx;
                break;
            }
        }
        if del_idx == webhooks.len() + 1 {
            Err(MyPlexError {})
        } else {
            webhooks.remove(del_idx);
            self.set_webhooks(webhooks)
        }
    }
}

impl From<serde_json::Error> for MyPlexError {
    fn from(e: serde_json::Error) -> Self {
        println!("{:#?}", e);
        Self {}
    }
}
