use super::get_last_plex_tags::{DOCKER_PLEX_IMAGE_NAME, DOCKER_PLEX_IMAGE_TAG_LATEST};
use crate::flags;
use plex_api::MyPlexBuilder;
use std::io::Write;
use testcontainers::{clients, core::WaitFor, images::generic::GenericImage, RunnableImage};
use xshell::{cmd, cwd, pushenv};

impl flags::Test {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        self.validate()?;

        let _plex_token = pushenv("PLEX_API_AUTH_TOKEN", "");

        if !self.online {
            cmd!("cargo test --workspace --no-fail-fast --features tests_deny_unknown_fields")
                .run()?;
        }

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
                self.integration_tests("", "")?;
            }

            match self.token.as_ref() {
                Some(token) if !token.is_empty() => {
                    cmd!("cargo xtask plex-data --replace").run()?;
                    let _plex_token = pushenv("PLEX_API_AUTH_TOKEN", token);

                    print!("// Requesting claim token... ");
                    let _ = std::io::stdout().flush();

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

                    println!("done!");
                    let _ = std::io::stdout().flush();

                    self.integration_tests(&claim_token.to_string(), token)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn integration_tests(&self, claim_token: &str, auth_token: &str) -> anyhow::Result<()> {
        let image_tag = self
            .docker_tag
            .clone()
            .unwrap_or_else(|| DOCKER_PLEX_IMAGE_TAG_LATEST.to_owned());
        let docker_image: RunnableImage<GenericImage> =
            GenericImage::new(DOCKER_PLEX_IMAGE_NAME, &image_tag)
                .with_wait_for(WaitFor::Healthcheck)
                .into();

        #[cfg_attr(windows, allow(unused_mut))]
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

        print!("// Spawning docker container... ");
        let _ = std::io::stdout().flush();

        let _plex_node = docker.run(docker_image);

        println!("done!");
        let _ = std::io::stdout().flush();

        let server_url = format!("http://localhost:{}/", _plex_node.get_host_port(32400));
        cmd!("cargo run -p plex-cli --  --server {server_url} --token {auth_token} wait").run()?;

        let _plex_server = pushenv("PLEX_SERVER_URL", server_url);

        let mut features = {
            if claim_token.is_empty() {
                "tests_only_online_unclaimed_server".to_owned()
            } else {
                "tests_only_online_claimed_server".to_owned()
            }
        };

        if self.deny_unknown_fields {
            features.push_str(",tests_deny_unknown_fields");
        }

        let mut test_run_result =
            cmd!("cargo test --workspace --no-fail-fast --features {features}").run();

        // Claim/Unclaim must be executed after all the other tests.
        // Not including `tests_deny_unknown_fields` feature here to avoid
        // possible unneeded failures.
        if !claim_token.is_empty() {
            let unclaim = cmd!("cargo test --workspace --no-fail-fast --features tests_only_online_claimed_server --test server online::unclaim_server -- --include-ignored --exact").run();
            test_run_result = test_run_result.and(unclaim);
            let claim = cmd!("cargo test --workspace --no-fail-fast --features tests_only_online_unclaimed_server --test server online::claim_server -- --include-ignored --exact").run();
            test_run_result = test_run_result.and(claim);

            // Running the `unclaim` once again to remove the freshly claimed
            // server from the account.
            let unclaim = cmd!("cargo test --workspace --no-fail-fast --features tests_only_online_unclaimed_server --test server online::unclaim_server -- --include-ignored --exact").run();
            test_run_result = test_run_result.and(unclaim);
        }

        test_run_result.map_err(|e| e.into())
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
