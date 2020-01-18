use clap::ArgMatches;
use plex_api::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum SubcommandSettingsError {
    #[error("We were unable to get the '{name}' current value, details: {source}")]
    CantFindSetting { name: String, source: PlexApiError },
    #[error("We were unable to update '{name}' with new value '{value:?}', details: {source}")]
    UnableToUpdateValue {
        name: String,
        value: String,
        source: PlexApiError,
    },
    #[error("We were unable to update '{name}' with new value '{value:?}', details: {source}")]
    UnableToUpdateOnServer {
        name: String,
        value: String,
        source: PlexApiError,
    },
}

pub(crate) async fn subcommand_settings(
    token: &str,
    matches: Option<&ArgMatches<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = matches.unwrap();
    let server_url = matches.value_of("server-url").unwrap();
    let srv = {
        if token.is_empty() {
            Server::connect(server_url).await
        } else {
            Server::connect_auth(server_url, token).await
        }
    }?;

    let setting_name = matches.value_of("name").unwrap();

    let mut settings = srv.get_settings().await?;

    match settings.get(setting_name) {
        Ok(setting) => {
            if matches.is_present("get") {
                println!(
                    "Current value of '{}' is '{}'",
                    setting_name,
                    setting.get_value().to_string()
                );
                Ok(())
            } else if let Some(value) = matches.value_of("set") {
                let new_value = match setting.get_value() {
                    SettingValue::Bool(_) => {
                        SettingValue::Bool(value == "1" || value == "true" || value == "yes")
                    }
                    SettingValue::Text(_) => SettingValue::Text(String::from(value)),
                    SettingValue::Int(_) => SettingValue::Int(value.parse().unwrap()),
                    SettingValue::Double(_) => SettingValue::Double(value.parse().unwrap()),
                };
                match settings.set(setting_name, new_value) {
                    Ok(_) => match srv.update_settings(&settings).await {
                        Ok(_) => {
                            println!("Updated successfully");
                            Ok(())
                        }
                        Err(e) => Err(Box::new(SubcommandSettingsError::UnableToUpdateOnServer {
                            name: String::from(setting_name),
                            value: String::from(value),
                            source: e,
                        })),
                    },
                    Err(e) => Err(Box::new(SubcommandSettingsError::UnableToUpdateValue {
                        name: String::from(setting_name),
                        value: String::from(value),
                        source: e,
                    })),
                }
            } else {
                Ok(())
            }
        }
        Err(e) => Err(Box::new(SubcommandSettingsError::CantFindSetting {
            name: String::from(setting_name),
            source: e,
        })),
    }
}
