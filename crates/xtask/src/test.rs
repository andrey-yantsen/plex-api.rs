use crate::{
    flags,
    get_last_plex_tags::{DOCKER_PLEX_IMAGE_NAME, DOCKER_PLEX_IMAGE_TAG_MIN_SUPPORTED},
};
use anyhow::bail;
use plex_api::{
    media_container::devices::Feature,
    sharing::{
        Filters, Friend, InviteStatus, Permissions, ShareableLibrary, ShareableServer, User,
    },
    HttpClientBuilder, MyPlex, MyPlexBuilder, Server,
};
use std::{io::Write, time::Duration};
use testcontainers::{clients, core::WaitFor, images::generic::GenericImage, RunnableImage};
use tokio::{runtime::Runtime, time::sleep};
use xshell::{cmd, Shell};

impl flags::Test {
    pub(crate) fn run(self, sh: &Shell) -> anyhow::Result<()> {
        self.validate()?;

        let _plex_token = sh.push_env("PLEX_API_AUTH_TOKEN", "");

        if !self.online {
            let test_name = self.test_name.clone().unwrap_or_default();
            cmd!(
                sh,
                "cargo test --workspace --no-fail-fast --features tests_deny_unknown_fields {test_name}"
            )
            .run()?;
        }

        if !self.offline {
            let client_id = if let Some(client_id) = self.client_id.as_ref() {
                client_id.to_owned()
            } else {
                let random_uuid = uuid::Uuid::new_v4();
                random_uuid.to_string()
            };

            let _client_id = sh.push_env("X_PLEX_CLIENT_IDENTIFIER", client_id);

            let plex_data_path = if let Some(plex_data_path) = self.plex_data_path.as_ref() {
                plex_data_path.to_owned()
            } else {
                match self.docker_tag.as_ref() {
                    Some(t) => format!("plex-data-{t}"),
                    None => "plex-data".to_owned(),
                }
            };

            if !self.online || self.token.is_none() {
                cmd!(
                    sh,
                    "cargo xtask plex-data --replace --plex-data-path {plex_data_path}"
                )
                .run()?;

                Runtime::new()?.block_on(self.integration_tests(
                    sh,
                    &plex_data_path,
                    "",
                    "",
                    "",
                ))?;
            }

            if let Some(token) = self.token.as_ref() {
                cmd!(
                    sh,
                    "cargo xtask plex-data --replace --plex-data-path {plex_data_path}"
                )
                .run()?;

                let server_owner_token = self.server_owner_token.as_ref().unwrap_or(token);

                if !server_owner_token.is_empty() {
                    print!("// Requesting claim token... ");
                    let _ = std::io::stdout().flush();
                }

                Runtime::new()?.block_on(async {
                    let claim_token = if server_owner_token.is_empty() {
                        "".to_string()
                    } else {
                        MyPlexBuilder::default()
                            .set_token(server_owner_token.to_owned())
                            .set_test_token_auth(false)
                            .build()
                            .await
                            .expect("failed to build the MyPlex client")
                            .claim_token()
                            .await
                            .expect("failed to get claim token")
                            .to_string()
                    };

                    if !server_owner_token.is_empty() {
                        println!("done!");
                        let _ = std::io::stdout().flush();
                    }

                    self.integration_tests(
                        sh,
                        &plex_data_path,
                        &claim_token,
                        token,
                        server_owner_token,
                    )
                    .await
                })?;
            }
        }

        Ok(())
    }

    async fn find_friend(
        &self,
        myplex: &MyPlex,
        invite_status: InviteStatus,
        account_id: u64,
    ) -> Option<Friend> {
        let friends = myplex.sharing().unwrap().friends(invite_status).await;
        if let Ok(friends) = friends {
            if let Some(friend) = friends.into_iter().find(|friend| friend.id == account_id) {
                return Some(friend);
            }
        }

        None
    }

    #[must_use = "Method returns an access token that must be used to access the shared server"]
    async fn share_server(
        &self,
        server_url: &str,
        owner_token: &str,
        guest_token: &str,
    ) -> anyhow::Result<String> {
        let server = Server::new(
            server_url,
            HttpClientBuilder::default()
                .set_x_plex_token(owner_token.to_owned())
                .build()?,
        )
        .await?;

        let owner = MyPlexBuilder::default()
            .set_token(owner_token.to_owned())
            .build()
            .await?;
        let owner_account_id = owner.account().unwrap().id;

        let guest = MyPlexBuilder::default()
            .set_token(guest_token.to_owned())
            .build()
            .await?;

        let libraries = server.libraries();
        let all_library_ids: Vec<ShareableLibrary> = libraries
            .iter()
            .map(|library| ShareableLibrary::LibraryId(library.id()))
            .collect();

        owner
            .sharing()
            .unwrap()
            .share(
                User::Account(&guest),
                ShareableServer::Server(&server),
                &all_library_ids,
                Permissions::default(),
                Filters::default(),
            )
            .await?;

        if self
            .find_friend(&guest, InviteStatus::Accepted, owner_account_id)
            .await
            .is_none()
        {
            let mut done = false;
            let mut attempt = 0;
            while attempt < 10 {
                if let Some(friend) = self
                    .find_friend(&guest, InviteStatus::PendingReceived, owner_account_id)
                    .await
                {
                    friend.accept().await.unwrap();
                    done = true;
                    break;
                }

                sleep(Duration::from_secs(1)).await;
                attempt += 1;
            }

            if !done {
                bail!("Unable to find the new friend request");
            }
        }

        let mut attempt = 0;
        let device_manager = guest.device_manager().unwrap();
        while attempt < 60 {
            let resources = device_manager.resources().await?;
            let shared_device = resources.into_iter().find(|device| {
                device.provides(Feature::Server)
                    && device.identifier() == server.machine_identifier()
            });

            if let Some(device) = shared_device {
                if let Some(access_token) = device.access_token() {
                    return Ok(access_token.to_owned());
                }
            }

            attempt += 1;
            sleep(Duration::from_secs(1)).await;
        }

        bail!("Unable to find the new shared server in the resources list");
    }

    async fn integration_tests(
        &self,
        sh: &Shell,
        plex_data_path: &str,
        claim_token: &str,
        auth_token: &str,
        server_owner_token: &str,
    ) -> anyhow::Result<()> {
        let image_tag = self
            .docker_tag
            .clone()
            .unwrap_or_else(|| DOCKER_PLEX_IMAGE_TAG_MIN_SUPPORTED.to_owned());
        let docker_image: RunnableImage<GenericImage> =
            GenericImage::new(DOCKER_PLEX_IMAGE_NAME, &image_tag)
                .with_wait_for(WaitFor::Healthcheck)
                .into();

        #[cfg_attr(windows, allow(unused_mut))]
        let mut docker_image = docker_image
            .with_volume((
                format!("{}/{}/media", sh.current_dir().display(), plex_data_path),
                "/data",
            ))
            .with_volume((
                format!(
                    "{}/{}/config/Library",
                    sh.current_dir().display(),
                    plex_data_path
                ),
                "/config/Library",
            ))
            .with_volume((
                format!(
                    "{}/{}/transcode",
                    sh.current_dir().display(),
                    plex_data_path
                ),
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

        print!("// Spawning docker container {DOCKER_PLEX_IMAGE_NAME}:{image_tag}... ");
        let _ = std::io::stdout().flush();

        let plex_node = docker.run(docker_image);

        println!("done!");
        let _ = std::io::stdout().flush();

        let server_url = format!("http://localhost:{}/", plex_node.get_host_port_ipv4(32400));
        println!("Waiting for the plex server to boot...");
        cmd!(
            sh,
            "cargo run -q -p plex-cli --  --server {server_url} --token {server_owner_token} wait --full"
        )
        .run()?;

        let mut server_auth_token = auth_token.to_owned();
        if !server_owner_token.is_empty() && server_owner_token != auth_token {
            print!("// Sharing the server with another user... ");
            let _ = std::io::stdout().flush();

            let shared_server_access_token = self
                .share_server(&server_url, server_owner_token, auth_token)
                .await?;

            server_auth_token = shared_server_access_token.clone();

            if self.github_actions {
                println!();
                // Simply printing this line didn't worked out, trying strange magic
                sh.cmd("echo")
                    .arg(format!("::add-mask::{shared_server_access_token}"))
                    .quiet()
                    .run()?;
            }

            println!("done!");

            println!("// Waiting for the permission propogation...");

            cmd!(
                sh,
                "cargo run -q -p plex-cli --  --server {server_url} --token {shared_server_access_token} wait"
            )
            .run()?;
        }

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

        if !server_auth_token.is_empty() && server_auth_token != auth_token {
            features.push_str(",tests_shared_server_access_token");
        }

        let _plex_token = sh.push_env("PLEX_API_AUTH_TOKEN", server_auth_token);
        let _plex_server = sh.push_env("PLEX_SERVER_URL", server_url);

        let test_name = self.test_name.clone().unwrap_or_default();
        // Tests against a live server can change the server state while running
        // so running more than one test in parallel can cause conflicts.
        let mut test_run_result = cmd!(
            sh,
            "cargo test --workspace --no-fail-fast --features {features} {test_name} -- --test-threads=1"
        )
        .run();

        // Claim/Unclaim must be executed after all the other tests.
        // Not including `tests_deny_unknown_fields` feature here to avoid
        // possible unneeded failures.
        if !claim_token.is_empty() && option_env!("TESTCONTAINERS") != Some("keep") {
            let _plex_token = sh.push_env("PLEX_API_AUTH_TOKEN", server_owner_token);

            let unclaim = cmd!(sh, "cargo test --workspace --no-fail-fast --features tests_only_online_claimed_server --test server online::unclaim_server -- --include-ignored --exact").run();
            test_run_result = test_run_result.and(unclaim);
            let claim = cmd!(sh, "cargo test --workspace --no-fail-fast --features tests_only_online_unclaimed_server --test server online::claim_server -- --include-ignored --exact").run();
            test_run_result = test_run_result.and(claim);

            // Running the `unclaim` once again to remove the freshly claimed
            // server from the account.
            let unclaim = cmd!(sh, "cargo test --workspace --no-fail-fast --features tests_only_online_unclaimed_server --test server online::unclaim_server -- --include-ignored --exact").run();
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

        if self.token.is_some() && self.token.as_ref().unwrap().is_empty() {
            return Err(xflags::Error::new("`--token` can't be empty"));
        }

        if self.server_owner_token.is_some() && self.token.is_none() {
            return Err(xflags::Error::new(
                "`--token` must be provided when you set `--server-owner-token`",
            ));
        }
        Ok(())
    }
}
