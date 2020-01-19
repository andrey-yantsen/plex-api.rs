use clap::ArgMatches;
use plex_api::prelude::*;

pub(crate) async fn subcommand_unclaim(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let server_url = matches.value_of("server-url").unwrap();
    let mut srv = Server::connect(server_url, token).await?;

    match srv.unclaim().await {
        Ok(_) => {
            println!("Unclaimed successfully");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
