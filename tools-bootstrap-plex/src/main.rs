use plex_api::MyPlexAccount;

extern crate chrono;
extern crate fern;

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

    let token = std::env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
    let myplex = MyPlexAccount::by_token(&token).await.unwrap();

    dbg!(myplex.get_webhooks().await.expect("Unable to get webhooks"));

    Ok(())
}
