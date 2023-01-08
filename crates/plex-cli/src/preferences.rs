use crate::flags;
use plex_api::{HttpClientBuilder, SettingValue};

impl flags::Preferences {
    pub(crate) async fn run(&self, server: &str, auth_token: &str) -> anyhow::Result<()> {
        let client = HttpClientBuilder::default()
            .set_x_plex_token(auth_token.to_string())
            .build()?;
        let server = plex_api::Server::new(server, client.clone()).await?;
        let preferences = server.preferences().await?;
        match &self.subcommand {
            flags::PreferencesCmd::Get(cmd) => {
                if let Some(key) = &cmd.key {
                    let setting = preferences.get(key);
                    if let Some(setting) = setting {
                        println!("{} = {:?}", key, setting.value);
                    } else {
                        anyhow::bail!("Unknown setting: {}", key);
                    }
                } else {
                    for setting in preferences.all() {
                        if setting.advanced && !cmd.show_advanced {
                            continue;
                        }

                        if setting.hidden && !cmd.show_hidden {
                            continue;
                        }

                        if let Some(group) = &cmd.group {
                            if &setting.group != group {
                                continue;
                            }
                        }

                        if !setting.summary.is_empty() {
                            println!("{}", setting.summary);
                        }

                        print!("{} = {:?} (", setting.id, setting.value);

                        if !setting.group.is_empty() {
                            print!("group: {}, ", setting.group);
                        }

                        println!(
                            "advanced: {}, hidden: {})\n",
                            setting.advanced, setting.hidden
                        );
                    }
                }
            }
            flags::PreferencesCmd::Set(cmd) => {
                let setting = preferences.get(&cmd.key);

                if let Some(setting) = setting {
                    if setting.hidden && !cmd.i_know_what_i_am_doing {
                        anyhow::bail!("You're trying to modify a hidden parameter! Please confirm that you're sure with --i-know-what-i-am-doing argument.");
                    }

                    let value = match &setting.value {
                        SettingValue::Int(_) => SettingValue::Int(cmd.value.parse()?),
                        SettingValue::Text(_) => SettingValue::Text(cmd.value.clone()),
                        SettingValue::Bool(_) => SettingValue::Bool(cmd.value.parse()?),
                        SettingValue::Double(_) => SettingValue::Double(cmd.value.parse()?),
                    };

                    let mut preferences = preferences;
                    preferences.set(&cmd.key, value)?;
                    preferences.commit().await?;
                } else {
                    anyhow::bail!("Unknown setting: {}", cmd.key);
                }
            }
        }

        Ok(())
    }
}
