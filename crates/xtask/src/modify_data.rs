use crate::{flags, get_last_plex_tags::DOCKER_PLEX_IMAGE_NAME, utils::copy_tree};
use std::{fs::remove_dir_all, io::Write};
use testcontainers::{clients, core::WaitFor, images::generic::GenericImage, RunnableImage};
use xshell::{cmd, cwd};

const DEFAULT_TAG: &str = "1.26.2.5797-5bd057d2b";

impl flags::ModifyData {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        // First rebuild the data if desired
        if !self.no_replace {
            let init_data = flags::PlexData {
                plex_data_path: self.plex_data_path.clone(),
                replace: true,
                verbose: self.verbose,
            };

            init_data.run()?;
        }

        let path = match self.plex_data_path.as_ref() {
            Some(path) => path.as_str(),
            None => "plex-data",
        };

        let plex_data_path = cwd()?.join(path);
        let plex_config_path = plex_data_path
            .join("config")
            .join("Library")
            .join("Application Support")
            .join("Plex Media Server");
        let plex_stub_config_path = cwd()?
            .join("plex-stub-data")
            .join("config")
            .join("Library")
            .join("Application Support")
            .join("Plex Media Server");

        let image_tag = self
            .docker_tag
            .clone()
            .unwrap_or_else(|| DEFAULT_TAG.to_owned());

        let docker_image: RunnableImage<GenericImage> =
            GenericImage::new(DOCKER_PLEX_IMAGE_NAME, &image_tag)
                .with_wait_for(WaitFor::Healthcheck)
                .into();

        #[cfg_attr(windows, allow(unused_mut))]
        let mut docker_image = docker_image
            .with_volume((format!("{}/media", plex_data_path.display()), "/data"))
            .with_volume((
                format!("{}/config/Library", plex_data_path.display()),
                "/config/Library",
            ))
            .with_volume((
                format!("{}/transcode", plex_data_path.display()),
                "/transcode",
            ))
            .with_env_var(("TZ", "UTC"));

        #[cfg(not(windows))]
        {
            let uid = users::get_current_uid();
            let gid = users::get_current_gid();

            docker_image = docker_image
                .with_env_var(("PLEX_UID", uid.to_string()))
                .with_env_var(("PLEX_GID", gid.to_string()));
        }

        let docker = clients::Cli::default();

        print!("// Spawning docker container... ");
        let _ = std::io::stdout().flush();

        let plex_node = docker.run(docker_image);

        println!("done!");
        let _ = std::io::stdout().flush();

        let server_url = format!("http://localhost:{}/", plex_node.get_host_port_ipv4(32400));
        cmd!("cargo run -p plex-cli --  --server {server_url} wait").run()?;

        print!("// Server is running at {server_url}web/index.html. Press enter when done...");
        let _ = std::io::stdout().flush();

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();

        print!("// Stopping docker container... ");
        let _ = std::io::stdout().flush();
        plex_node.stop();
        println!("done!");

        remove_dir_all(&plex_stub_config_path)?;

        copy_tree(
            &plex_config_path,
            &plex_stub_config_path,
            vec!["Media"],
            self.verbose,
        )?;
        copy_tree(
            &plex_config_path,
            &plex_stub_config_path,
            vec!["Metadata"],
            self.verbose,
        )?;
        copy_tree(
            &plex_config_path,
            &plex_stub_config_path,
            vec!["Plug-in Support", "Databases"],
            self.verbose,
        )?;

        Ok(())
    }
}
