use std::{io::SeekFrom, path::Path};

use crate::flags;
use anyhow::bail;
use indicatif::{ProgressBar, ProgressStyle};
use plex_api::{
    library::{Item, Library, MediaItem},
    Server,
};
use tokio::{fs::OpenOptions, io::AsyncSeekExt};
use tokio_util::compat::TokioAsyncWriteCompatExt;

impl flags::Download {
    pub(crate) async fn run(&self, server: Server, library: Library) -> anyhow::Result<()> {
        if let Some(item_id) = self.item_id {
            match server.item_by_id(item_id).await {
                Ok(item) => return self.download_item(item).await,
                Err(error) => bail!("Failed to get requested item! {error}"),
            };
        }

        match library {
            Library::Movie(_) => todo!(),
            Library::TV(_) => todo!(),
            Library::Music(_) => todo!(),
            Library::Video(_) => todo!(),
            Library::Photo(_) => todo!(),
        }
    }

    async fn download_item(&self, item: impl Into<Item>) -> anyhow::Result<()> {
        let item: Item = item.into();
        assert_eq!(1, item.media().len(), "Expected a single media!");
        let media = &item.media()[0];
        let parts = media.parts();

        if parts.is_empty() {
            bail!("Media doesn't have any part associated with it; nothing to download!");
        }

        if parts.len() > 1 && self.required_parts.is_empty() && !self.all_parts {
            bail!("Media has multiple parts. Please provide required part indices via --part, or approve downloading all parts via --all-parts.");
        }

        if !self.required_parts.is_empty() {
            if let Some(max_requested_part) = self.required_parts.iter().max() {
                if *max_requested_part > media.parts().len() + 1 {
                    bail!(
                        "Incorrect --part provided! The media has only {parts_cnt} parts.",
                        parts_cnt = media.parts().len()
                    );
                }
            }
        }

        let download_all_parts = self.all_parts || parts.len() == 1;
        let multiple_parts_expected = parts.len() > 1 && (self.all_parts || parts.len() > 1);

        for part in &parts {
            let metadata = part.metadata();

            if metadata.key.is_none() {
                eprintln!(
                    "Part {} doesn't have the key! Skipping it.",
                    part.part_index
                );
                continue;
            }

            if download_all_parts || self.required_parts.contains(&(part.part_index + 1)) {
                let destination_file_name = {
                    if let Some(remote_file_path) = &metadata.file {
                        Path::new(remote_file_path)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned()
                    } else {
                        let key = Path::new(&metadata.key.as_ref().unwrap())
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned();
                        if multiple_parts_expected {
                            format!(
                                "{}.part.{}.{}",
                                key,
                                part.part_index + 1,
                                metadata.container.unwrap()
                            )
                        } else {
                            format!("{}.{}", key, metadata.container.unwrap())
                        }
                    }
                };
                let mut target = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&destination_file_name)
                    .await?;

                let pos = target.seek(SeekFrom::End(0)).await?;

                if pos > 0 {
                    println!(
                        "Downloading part #{} to {destination_file_name} (resuming download)",
                        part.part_index + 1
                    );
                } else {
                    println!(
                        "Downloading part #{} to {destination_file_name}",
                        part.part_index + 1
                    );
                }

                let pb = ProgressBar::new(part.len().unwrap());
                pb.set_style(
                    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
                        .unwrap()
                        .progress_chars("#>-")
                );

                pb.set_position(pos);

                let mut wrapped_writer = pb.wrap_async_write(target).compat_write();

                part.download(&mut wrapped_writer, pos..).await?;
            } else {
                println!("Skipping part #{}", part.part_index + 1);
            }
        }

        Ok(())
    }
}
