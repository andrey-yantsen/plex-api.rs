//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

mod flags;
mod get_last_plex_tags;
mod modify_data;
mod plex_data;
mod test;
pub mod utils;

use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
};
use xshell::Shell;

fn main() -> Result<()> {
    let sh = Shell::new()?;
    let _d = sh.push_dir(project_root());

    let flags = flags::Xtask::from_env()?;

    match flags.subcommand {
        flags::XtaskCmd::Test(cmd) => cmd.run(&sh),
        flags::XtaskCmd::PlexData(cmd) => cmd.run(&sh),
        flags::XtaskCmd::ModifyData(cmd) => cmd.run(&sh),
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
