use clap::ArgMatches;
use plex_api::prelude::*;

pub(crate) async fn subcommand_claim(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let server_url = matches.value_of("server-url").unwrap();
    let mut srv = Server::connect(server_url, token).await?;

    let claim_token = {
        if let Some(claim_token) = matches.value_of("claim-token") {
            if !claim_token.starts_with("claim-") {
                panic!("Provided claim-token doesn`t looks like one, have you really got it from https://www.plex.tv/claim?")
            } else {
                claim_token.to_owned()
            }
        } else if token.is_empty() {
            panic!("You should specify either `--claim-token` or `--auth-token`");
        } else {
            match MyPlexAccount::by_token(token).await {
                Ok(acc) => match acc.get_claim_token().await {
                    Ok(claim_token) => claim_token,
                    Err(e) => return Err(Box::new(e)),
                },
                Err(e) => return Err(Box::new(e)),
            }
        }
    };

    match srv.claim(&claim_token).await {
        Ok(_) => {
            println!("Claimed successfully");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
