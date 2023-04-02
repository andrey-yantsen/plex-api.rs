use std::fmt::Formatter;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone)]
pub enum Guid {
    Local(String),
    Imdb(String),
    Tmdb(String),
    Tvdb(String),
    LastFm(String),
    Plex(String, String),
    None(String),
    Collection(String),
    Mbid(String),
    PlexMusic(String),
    Iva(String),
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    Unknown(String),
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Guid;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("plex-formatted guid string or object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Guid, E>
            where
                E: ::serde::de::Error,
            {
                let parts = value.split("://").collect::<Vec<&str>>();
                if parts.len() != 2 {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &"guid value formatted as protocol://path",
                    ));
                }

                Ok(match parts[..] {
                    ["imdb", id] | ["com.plexapp.agents.imdb", id] => Guid::Imdb(id.to_owned()),
                    ["local", id] => Guid::Local(id.to_owned()),
                    ["tvdb", id] | ["com.plexapp.agents.thetvdb", id] => Guid::Tvdb(id.to_owned()),
                    ["tmdb", id] => Guid::Tmdb(id.to_owned()),
                    ["collection", id] => Guid::Collection(id.to_owned()),
                    ["com.plexapp.agents.lastfm", id] => Guid::LastFm(id.to_owned()),
                    ["mbid", id] => Guid::Mbid(id.to_owned()),
                    ["com.plexapp.agents.none", id] => Guid::None(id.to_owned()),
                    ["com.plexapp.agents.plexmusic", id] => Guid::PlexMusic(id.to_owned()),
                    ["iva", id] => Guid::Iva(id.to_owned()),
                    ["plex", id] => {
                        let plex_guid_parts = id.split('/').collect::<Vec<&str>>();
                        if plex_guid_parts.len() != 2 {
                            return Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &"guid value formatted as plex://media_type/id",
                            ));
                        }

                        Guid::Plex(plex_guid_parts[0].to_owned(), plex_guid_parts[1].to_owned())
                    }
                    _ => {
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        {
                            Guid::Unknown(value.to_owned())
                        }
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        {
                            return Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &"see source code for supported values",
                            ));
                        }
                    }
                })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Guid, M::Error>
            where
                M: MapAccess<'de>,
            {
                if let Some((key, value)) = map.next_entry::<&str, &str>()? {
                    if key == "id" {
                        self.visit_str(value)
                    } else {
                        Err(serde::de::Error::unknown_field(
                            value,
                            &["object shouldn't have fields other than id"],
                        ))
                    }
                } else {
                    Err(serde::de::Error::missing_field("object must have id field"))
                }
            }
        }

        deserializer.deserialize_any(V)
    }
}
