#![allow(clippy::manual_map)]

mod download;

use crate::flags;
use plex_api::{HttpClientBuilder, Server};

impl flags::Library {
    pub(crate) async fn run(&self, host: &str, auth_token: &str) -> anyhow::Result<()> {
        let client = HttpClientBuilder::default()
            .set_x_plex_token(auth_token.to_string())
            .build()?;
        let server = Server::new(host, client.clone()).await?;

        let library = {
            if let Some(name) = self.name.as_ref() {
                Some(
                    server
                        .libraries()
                        .into_iter()
                        .find(|l| l.title() == name)
                        .expect("Unable to find required library"),
                )
            } else if let Some(id) = self.id.as_ref() {
                Some(
                    server
                        .libraries()
                        .into_iter()
                        .find(|l| l.id() == id)
                        .expect("Unable to find requested library"),
                )
            } else {
                None
            }
        };

        match &self.subcommand {
            flags::LibraryCmd::Download(cmd) => {
                cmd.run(server, library.expect("Library must be provided"))
                    .await
            }
        }
    }
}
