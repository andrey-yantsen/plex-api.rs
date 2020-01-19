extern crate chrono;
extern crate fern;
#[macro_use]
extern crate clap;

mod subcommands;

use clap::App;
use subcommands::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let yaml = load_yaml!("clap-config.yaml");
    let matches = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let token_from_env = std::env::var("PLEX_API_AUTH_TOKEN").unwrap_or_else(|_| String::from(""));
    let auth_token = matches.value_of("auth-token").unwrap_or(&token_from_env);

    match matches.subcommand() {
        ("wait", subcommand_matches) => subcommand_wait(auth_token, subcommand_matches).await,
        ("settings", subcommand_matches) => {
            subcommand_settings(auth_token, subcommand_matches).await
        }
        ("token", subcommand_matches) => subcommand_token(auth_token, subcommand_matches).await,
        ("unclaim", subcommand_matches) => subcommand_unclaim(auth_token, subcommand_matches).await,
        ("claim", subcommand_matches) => subcommand_claim(auth_token, subcommand_matches).await,
        _ => {
            // Unexpected subcommand called, we shouldn't even be here
            unimplemented!();
        }
    }
}
