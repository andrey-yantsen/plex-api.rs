#![cfg_attr(windows, allow(unused_imports))]

use crate::flags;
#[cfg(not(windows))]
use crate::plex_docker_image::Plex as PlexImage;
use plex_api::MyPlexBuilder;
#[cfg(not(windows))]
use testcontainers::{clients, RunnableImage};
use xshell::{cmd, cwd, pushenv};

impl flags::Test {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        self.validate()?;

        let _plex_token = pushenv("PLEX_API_AUTH_TOKEN", "");

        if !self.online {
            cmd!("cargo test --workspace --no-fail-fast --features deny_unknown_fields").run()?;
        }

        #[cfg(not(windows))]
        if !self.offline {
            let client_id = if let Some(client_id) = self.client_id.as_ref() {
                client_id.to_owned()
            } else {
                let random_uuid = uuid::Uuid::new_v4();
                random_uuid.to_string()
            };

            let _client_id = pushenv("X_PLEX_CLIENT_IDENTIFIER", &client_id);

            if !self.online || self.token.is_none() || self.token.as_ref().unwrap().is_empty() {
                cmd!("cargo xtask plex-data --replace").run()?;
                self.integration_tests("")?;
            }

            match self.token.as_ref() {
                Some(token) if !token.is_empty() => {
                    cmd!("cargo xtask plex-data --replace").run()?;
                    let _plex_token = pushenv("PLEX_API_AUTH_TOKEN", token);
                    let claim_token = tokio::runtime::Runtime::new()?.block_on(async {
                        MyPlexBuilder::default()
                            .set_token(token)
                            .set_test_token_auth(false)
                            .build()
                            .await
                            .expect("failed to build the MyPlex client")
                            .claim_token()
                            .await
                            .expect("failed to get claim token")
                    });
                    self.integration_tests(&claim_token.to_string())?;
                }
                _ => (),
            }
        }

        #[cfg(windows)]
        if self.online {
            anyhow::bail!("Online tests are not supported on Windows");
        }

        Ok(())
    }

    #[cfg(not(windows))]
    fn integration_tests(&self, claim_token: &str) -> anyhow::Result<()> {
        let docker_image: RunnableImage<PlexImage> = match self.docker_tag.as_ref() {
            Some(tag) => PlexImage::new(tag.to_owned()),
            None => PlexImage::default(),
        }
        .into();

        let mut docker_image = docker_image
            .with_volume((format!("{}/plex-data/media", cwd()?.display()), "/data"))
            .with_volume((
                format!("{}/plex-data/config/Library", cwd()?.display()),
                "/config/Library",
            ))
            .with_volume((
                format!("{}/plex-data/transcode", cwd()?.display()),
                "/transcode",
            ))
            .with_env_var(("TZ", "UTC"))
            .with_env_var(("PLEX_CLAIM", claim_token));

        #[cfg(not(windows))]
        {
            let uid = users::get_current_uid();
            let gid = users::get_current_gid();

            docker_image = docker_image
                .with_env_var(("PLEX_UID", uid.to_string()))
                .with_env_var(("PLEX_GID", gid.to_string()));
        }

        let docker = clients::Cli::default();
        let _plex_node = docker.run(docker_image);

        let _plex_server = pushenv(
            "PLEX_SERVER_URL",
            format!("http://localhost:{}/", _plex_node.get_host_port(32400)),
        );

        let mut features = "tests_only_online".to_owned();

        if self.deny_unknown_fields {
            features.push_str(",deny_unknown_fields");
        }

        cmd!("cargo test --workspace --no-fail-fast --features {features}").run()?;

        Ok(())
    }

    fn validate(&self) -> xflags::Result<()> {
        if self.offline && self.online {
            return Err(xflags::Error::new(
                "`--offline` and `--only-authenticated` can't be specified at the same time",
            ));
        }
        if self.offline && self.deny_unknown_fields {
            return Err(xflags::Error::new(
                "`--offline` and `--deny-unknown-fields` can't be specified at the same time",
            ));
        }
        Ok(())
    }
}
