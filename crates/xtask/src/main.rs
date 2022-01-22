//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

mod flags;
mod get_last_plex_tags;
mod plex_data;
mod test;

use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
};
use xshell::pushd;

fn main() -> Result<()> {
    let _d = pushd(project_root())?;

    let flags = flags::Xtask::from_env()?;

    match flags.subcommand {
        flags::XtaskCmd::Help(_) => {
            println!("{}", flags::Xtask::HELP);
            Ok(())
        }
        flags::XtaskCmd::Test(cmd) => cmd.run(),
        flags::XtaskCmd::PlexData(cmd) => cmd.run(),
        flags::XtaskCmd::GetLastPlexTags(cmd) => cmd.run(),
    }
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .to_path_buf()
}
