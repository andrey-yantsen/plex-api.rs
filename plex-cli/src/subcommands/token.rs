use clap::ArgMatches;
use plex_api::MyPlexAccount;

pub(crate) async fn subcommand_token(
    token: &str,
    _matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let my_plex = MyPlexAccount::by_token(token).await?;
    let claim_token = my_plex.get_claim_token().await?;
    println!("{}", claim_token);
    Ok(())
}
