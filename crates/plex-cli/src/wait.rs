use crate::flags;
use plex_api::HttpClientBuilder;

impl flags::Wait {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let client = {
            let mut builder = HttpClientBuilder::default();
            if let Some(token) = &self.token {
                builder = builder.set_x_plex_token(token.to_string());
            }
            builder.build()?
        };
        let server_address = self
            .server
            .clone()
            .unwrap_or_else(|| "http://127.0.0.1:32400".to_owned());

        let start_time = std::time::SystemTime::now();
        let time_limit = std::time::Duration::from_secs(self.timeout.unwrap_or(120) as u64);
        let sleep_duration = std::time::Duration::from_secs(self.delay.unwrap_or(1) as u64);

        while start_time.elapsed()? < time_limit {
            let server_result = plex_api::Server::new(server_address.clone(), client.clone()).await;
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
