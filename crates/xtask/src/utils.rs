use std::{
    fs::{copy, create_dir_all, hard_link},
    path::Path,
};

use pathdiff::diff_paths;
use walkdir::WalkDir;

pub const DOCKER_TAG_MIN_SUPPORTED: &str = "1.27.2.5929-a806c5905";

fn symlink<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    path: P,
    target: Q,
    new_path: R,
    verbose: bool,
) -> anyhow::Result<()> {
    if let Some(path_parent) = path.as_ref().parent() {
        if let Some(relative) = diff_paths(&target, path_parent) {
            if let Some(parent) = new_path.as_ref().parent() {
                if !parent.exists() {
                    create_dir_all(parent)?;
                }
            }

            if verbose {
                println!(
                    "Linking '{}' to '{}'",
                    new_path.as_ref().display(),
                    relative.display()
                );
            }

            #[cfg(not(windows))]
            std::os::unix::fs::symlink(relative, new_path)?;

            #[cfg(windows)]
            std::os::windows::fs::symlink_file(relative, new_path)?;

            return Ok(());
        }
    }

    eprintln!(
        "Could not relink '{}', falling back to copying.",
        path.as_ref().display(),
    );

    cp(false, path, new_path, verbose)
}

pub fn cp<P: AsRef<Path>, Q: AsRef<Path>>(
    hardlink_suported: bool,
    original: P,
    link: Q,
    verbose: bool,
) -> anyhow::Result<()> {
    if link.as_ref().exists() {
        if verbose {
            println!("File '{}' already exists!", link.as_ref().display());
        }
        return Ok(());
    }

    if let Some(parent) = link.as_ref().parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    if hardlink_suported {
        if verbose {
            println!(
                "Copying '{}' to '{}'",
                original.as_ref().display(),
                link.as_ref().display()
            );
        }

        hard_link(original, link)?;
    } else {
        if verbose {
            println!(
                "Copying '{}' to '{}'",
                original.as_ref().display(),
                link.as_ref().display()
            );
        }

        copy(original, link)?;
    }

    Ok(())
}

/// Copies files within a `source_root` to the same location in the `target_root` maintaining any
/// symlinks that point within the source_root. `inner_paths` defines the subtree to copy.
pub fn copy_tree(
    source_root: &Path,
    target_root: &Path,
    inner_paths: Vec<&str>,
    verbose: bool,
) -> anyhow::Result<()> {
    let mut source = source_root.to_owned();
    let mut target = target_root.to_owned();

    for path in inner_paths {
        source = source.join(path);
        target = target.join(path);
    }

    for entry in WalkDir::new(&source)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            let new_path = target.join(path.strip_prefix(&source)?);

            cp(false, entry.path(), new_path, verbose)?;
        } else if entry.file_type().is_symlink() {
            let path = entry.path();
            let new_path = target_root.join(path.strip_prefix(source_root)?);
            let link_target = path.canonicalize()?;

            if link_target.strip_prefix(source_root).is_ok() {
                symlink(path, &link_target, &new_path, verbose)?;
            } else {
                cp(false, path, new_path, verbose)?;
            }
        }
    }

    Ok(())
}
