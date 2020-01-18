use clap::ArgMatches;
use plex_api::Server;

pub(crate) async fn subcommand_unclaim(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let server_url = matches.value_of("server-url").unwrap();
    let mut srv = {
        if token.is_empty() {
            Server::connect(server_url).await
        } else {
            Server::connect_auth(server_url, token).await
        }
    }?;

    match srv.unclaim().await {
        Ok(_) => {
            println!("Unclaimed successfully");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
