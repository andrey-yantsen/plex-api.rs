use clap::ArgMatches;
use plex_api::prelude::*;
use std::thread;
use std::time::Duration;
use tokio::time;

async fn wait(
    token: &str,
    server_url: &str,
    wait_for_settings: bool,
    delay: &Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let srv = Server::connect(server_url, token).await;
        match srv {
            Ok(srv) => {
                if wait_for_settings {
                    match srv.get_settings().await {
                        Ok(settings) => match settings.get("AcceptedEULA") {
                            Ok(..) => {
                                break;
                            }
                            Err(e) => {
                                eprintln!("AcceptedEULA setting not found: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Unable to get settings: {}", e);
                        }
                    }
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to the server: {}", e);
            }
        }
        thread::sleep(*delay);
    }
    Ok(())
}

pub(crate) async fn subcommand_wait(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let timeout = Duration::from_secs(matches.value_of("timeout").unwrap_or("30").parse().unwrap());
    let server_url = matches.value_of("server-url").unwrap();
    let wait_for_settings = matches.is_present("wait-for-settings");
    let delay = Duration::from_secs(matches.value_of("delay").unwrap_or("3").parse().unwrap());

    let result = time::timeout(timeout, wait(token, server_url, wait_for_settings, &delay)).await;

    if result.is_err() {
        Err(Box::new(result.err().unwrap()))
    } else {
        Ok(())
    }
}
