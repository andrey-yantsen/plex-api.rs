mod flags;
mod wait;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = flags::PlexCli::from_env()?;

    match flags.subcommand {
        flags::PlexCliCmd::Help(cmd) => cmd.run(),
        flags::PlexCliCmd::Wait(cmd) => cmd.run().await,
    }
}
