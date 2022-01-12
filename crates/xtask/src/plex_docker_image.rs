#![cfg_attr(windows, allow(dead_code))]

#[cfg(not(windows))]
use testcontainers::{core::WaitFor, Image};

pub const NAME: &str = "plexinc/pms-docker";
pub const TAG_LATEST: &str = "latest";

#[derive(Debug, Clone)]
pub struct Plex {
    tag: String,
}

impl Default for Plex {
    fn default() -> Self {
        Self {
            tag: TAG_LATEST.to_owned(),
        }
    }
}

impl Plex {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }
}

#[cfg(not(windows))]
impl Image for Plex {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![
            WaitFor::message_on_stdout("[services.d] done."),
            WaitFor::seconds(10),
        ]
    }
}
