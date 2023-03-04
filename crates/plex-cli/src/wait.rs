use crate::flags;
use plex_api::HttpClientBuilder;
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

        while start_time.elapsed()? < time_limit {
            let server_result = plex_api::Server::new(host, client.clone()).await;
            if let Ok(server) = server_result {
                // The `start_state` is None when the server has finished loading.
                if server.media_container.start_state.is_none() || !wait_full_start {
                    let prefs = server.preferences().await;
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

        anyhow::bail!("Timeout reached")
    }
}
