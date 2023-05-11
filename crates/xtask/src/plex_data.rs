use crate::{
    flags,
    utils::{copy_tree, cp},
};
use std::{
    fs::{create_dir_all, hard_link, remove_dir_all, remove_file},
    path::Path,
};
use xshell::Shell;

const STUB_PICTURE: &str = "white_noise_720p.jpg";
const STUB_VIDEO_1: &str = "testsrc_720p_h264_v1.mkv";
const STUB_VIDEO_2: &str = "testsrc_720p_h264_v2.mkv";
const STUB_VIDEO_3: &str = "white_noise_720p_h265.mp4";
const STUB_AUDIO: &str = "white_noise.aac";

const LIBRARY_VIDEOS_1: [(&str, &str); 7] = [
    ("Movies", "Batman (1989).mkv"),
    ("Movies", "Big Buck Bunny (2008) - part1.mkv"),
    ("Movies", "Big Buck Bunny (2008) - part2.mkv"),
    ("Movies", "Elephants Dream (2006).mkv"),
    ("Movies", "Sintel (2010).mkv"),
    ("Movies", "Interstate 60 (2002).mkv"),
    (
        "TV-Shows/The Big Bang Theory",
        "The.Big.Bang.Theory.S01E01.mkv",
    ),
];

const LIBRARY_VIDEOS_2: [(&str, &str); 1] = [(
    "TV-Shows/The Big Bang Theory",
    "The.Big.Bang.Theory.S01E02.mkv",
)];

const LIBRARY_VIDEOS_3: [(&str, &str); 37] = [
    ("Movies", "Elephants Dream (2006).mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E01.mkv"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E02.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E03.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E04.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E05.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E06.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E07.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E08.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S01E09.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E01.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E02.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E03.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E04.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E05.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E06.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E07.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E08.mp4"),
    ("TV-Shows/Game of Thrones", "Game.of.Thrones.S02E09.mp4"),
    ("TV-Shows/The 100", "The.100.S01E01.mp4"),
    ("TV-Shows/The 100", "The.100.S01E02.mp4"),
    ("TV-Shows/The 100", "The.100.S01E03.mp4"),
    ("TV-Shows/The 100", "The.100.S01E04.mp4"),
    ("TV-Shows/The 100", "The.100.S01E05.mp4"),
    ("TV-Shows/The 100", "The.100.S01E06.mp4"),
    ("TV-Shows/The 100", "The.100.S01E07.mp4"),
    ("TV-Shows/The 100", "The.100.S01E08.mp4"),
    ("TV-Shows/The 100", "The.100.S01E09.mp4"),
    ("TV-Shows/The 100", "The.100.S02E01.mp4"),
    ("TV-Shows/The 100", "The.100.S02E02.mp4"),
    ("TV-Shows/The 100", "The.100.S02E03.mp4"),
    ("TV-Shows/The 100", "The.100.S02E04.mp4"),
    ("TV-Shows/The 100", "The.100.S02E05.mp4"),
    ("TV-Shows/The 100", "The.100.S02E06.mp4"),
    ("TV-Shows/The 100", "The.100.S02E07.mp4"),
    ("TV-Shows/The 100", "The.100.S02E08.mp4"),
    ("TV-Shows/The 100", "The.100.S02E09.mp4"),
];

const LIBRARY_PICTURES: [(&str, &str); 9] = [
    ("Photos/Cats", "Picture1.jpg"),
    ("Photos/Cats", "Picture2.jpg"),
    ("Photos/Cats", "Picture3.jpg"),
    ("Photos/Cats/Cats in bed", "Picture1.jpg"),
    ("Photos/Cats/Cats in bed", "Picture2.jpg"),
    ("Photos/Cats/Cats in bed", "Picture3.jpg"),
    ("Photos/Cats/Cats not in bed", "Picture1.jpg"),
    ("Photos/Cats/Cats not in bed", "Picture2.jpg"),
    ("Photos/Cats/Cats not in bed", "Picture3.jpg"),
];

const LIBRARY_AUDIO: [(&str, &str); 9] = [
    (
        "Music/System of a Down - Toxicity (1999)",
        "01 - Toxicity.aac",
    ),
    (
        "Music/System of a Down - Toxicity (1999)",
        "02 - Marmalade.aac",
    ),
    ("Music/System of a Down - Toxicity (1999)", "03 - Metro.aac"),
    (
        "Music/System of a Down - Aerials (2002)",
        "01 - Aerials.aac",
    ),
    (
        "Music/System of a Down - Aerials (2002)",
        "02 - Streamline (album version).aac",
    ),
    (
        "Music/System of a Down - Aerials (2002)",
        "03 - Sugar (live).aac",
    ),
    (
        "Music/Skrillex - Try It Out (2003)",
        "01 - TRY IT OUT (NEON MIX).aac",
    ),
    (
        "Music/Skrillex - Try It Out (2003)",
        "02 - Try It Out (Try Harder Mix).aac",
    ),
    (
        "Music/Skrillex - Try It Out (2003)",
        "03 - Try It Out (Put Em Up Mix).aac",
    ),
];

impl flags::PlexData {
    pub(crate) fn run(self, sh: &Shell) -> anyhow::Result<()> {
        let path = match self.plex_data_path.as_ref() {
            Some(path) => path.as_str(),
            None => "plex-data",
        };

        let plex_data_path = sh.current_dir().join(path);
        let plex_stub_data_path = sh.current_dir().join("plex-stub-data");
        let plex_stub_data_media_path = plex_stub_data_path.join("media");

        if self.replace && plex_data_path.exists() {
            remove_dir_all(&plex_data_path)?;
        }

        create_dir_all(&plex_data_path)?;

        let mut is_hardlink_supported = false;
        if let Ok(()) = hard_link("Cargo.lock", plex_data_path.join("Cargo.lock")) {
            remove_file(plex_data_path.join("Cargo.lock"))?;
            is_hardlink_supported = true;
        }

        copy_tree(
            &plex_stub_data_path,
            &plex_data_path,
            vec!["config"],
            self.verbose,
        )?;

        let media_path = plex_data_path.join("media");
        self.populate(
            is_hardlink_supported,
            &plex_stub_data_media_path.join(STUB_VIDEO_1),
            &media_path,
            &LIBRARY_VIDEOS_1,
        )?;
        self.populate(
            is_hardlink_supported,
            &plex_stub_data_media_path.join(STUB_VIDEO_2),
            &media_path,
            &LIBRARY_VIDEOS_2,
        )?;
        self.populate(
            is_hardlink_supported,
            &plex_stub_data_media_path.join(STUB_VIDEO_3),
            &media_path,
            &LIBRARY_VIDEOS_3,
        )?;
        self.populate(
            is_hardlink_supported,
            &plex_stub_data_media_path.join(STUB_PICTURE),
            &media_path,
            &LIBRARY_PICTURES,
        )?;
        self.populate(
            is_hardlink_supported,
            &plex_stub_data_media_path.join(STUB_AUDIO),
            &media_path,
            &LIBRARY_AUDIO,
        )?;

        Ok(())
    }

    fn populate(
        &self,
        is_hardlink_supported: bool,
        source: &Path,
        destination: &Path,
        paths: &[(&str, &str)],
    ) -> anyhow::Result<()> {
        for (dir, file) in paths.iter() {
            let dir = destination.join(dir);
            let dest = dir.join(file);

            cp(is_hardlink_supported, source, dest, self.verbose)?;
        }

        Ok(())
    }
}
