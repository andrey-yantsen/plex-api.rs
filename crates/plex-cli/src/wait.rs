use crate::flags;
use plex_api::{media_container::server::MappingState, HttpClientBuilder};
use std::time::{Duration, SystemTime};

impl flags::Wait {
    pub(crate) async fn run(&self, host: &str, auth_token: &str) -> anyhow::Result<()> {
        let client = HttpClientBuilder::default()
            .set_x_plex_token(auth_token.to_string())
            .build()?;
        let start_time = SystemTime::now();
        let wait_full_start = self.full;
        let time_limit = Duration::from_secs(self.timeout.unwrap_or({
            if wait_full_start {
                300
            } else {
                120
            }
        }) as u64);
        let sleep_duration = Duration::from_secs(self.delay.unwrap_or(1) as u64);

        let mut server_result = Err(plex_api::Error::ClientNotAuthenticated);
        let mut prefs = Err(plex_api::Error::ClientNotAuthenticated);
        while start_time.elapsed()? < time_limit {
            server_result = plex_api::Server::new(host, client.clone()).await;
            if let Ok(server) = &server_result {
                if !wait_full_start
                    || (server.media_container.start_state.is_none()
                        && (matches!(
                            server.media_container.my_plex_mapping_state,
                            MappingState::Mapped
                        ) || matches!(
                            server.media_container.my_plex_mapping_state,
                            MappingState::Unknown
                        )))
                {
                    prefs = server.preferences().await;
                    if prefs.is_ok() {
                        println!(
                            "Ready in {:.2} seconds!",
                            start_time.elapsed()?.as_secs_f32()
                        );
                        return Ok(());
                    }
                }
            }

            tokio::time::sleep(sleep_duration).await;
        }

        if self.verbose {
            println!("Details on the last attempt:");
            print!("Loading server details: ");

            if server_result.is_ok() {
                println!("success");

                let server = server_result.as_ref().unwrap();

                if wait_full_start {
                    println!("Start state: {:?}", server.media_container.start_state);

                    println!(
                        "MyPlex state: {:?}",
                        server.media_container.my_plex_mapping_state
                    );
                }
            } else {
                println!("failed ({:?})", server_result.err().unwrap());
            }

            if !matches!(prefs, Err(plex_api::Error::ClientNotAuthenticated)) {
                print!("Loading preferences: ");

                if prefs.is_ok() {
                    println!("success");
                } else {
                    println!("failed ({:?})", prefs.err().unwrap());
                }
            }
        }

        anyhow::bail!("Timeout reached")
    }
}
