use crate::flags;
use plex_api::HttpClientBuilder;

impl flags::Wait {
    pub(crate) async fn run(&self, server: &str, auth_token: &str) -> anyhow::Result<()> {
        let client = HttpClientBuilder::default()
            .set_x_plex_token(auth_token.to_string())
            .build()?;
        let start_time = std::time::SystemTime::now();
        let time_limit = std::time::Duration::from_secs(self.timeout.unwrap_or(120) as u64);
        let sleep_duration = std::time::Duration::from_secs(self.delay.unwrap_or(1) as u64);
        while start_time.elapsed()? < time_limit {
            let server_result = plex_api::Server::new(server, client.clone()).await;
            if let Ok(server) = server_result {
                let prefs = server.preferences().await;
                if prefs.is_ok() {
                    println!(
                        "Ready in {:.2} seconds!",
                        start_time.elapsed()?.as_secs_f32()
                    );
                    return Ok(());
                }
            }

            tokio::time::sleep(sleep_duration).await;
        }

        anyhow::bail!("Timeout reached")
    }
}
