mod friend;
mod server;

use crate::{
    media_container::users::AllowTuners,
    url::{MYPLEX_INVITES_FRIENDS, MYPLEX_INVITES_INVITE, MYPLEX_INVITES_SHARED_SERVERS},
    Library, MyPlex, Result, Server,
};
use serde::{
    de::{Deserializer, Visitor},
    Deserialize, Serialize,
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    result::Result as StdResult,
};

pub use friend::{Friend, InviteStatus};
pub use server::SharedServer;

#[derive(Debug)]
pub struct Sharing<'a> {
    myplex: &'a MyPlex,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    pub allow_sync: bool,
    pub allow_channels: bool,
    pub allow_tuners: AllowTuners,
    pub allow_subtitle_admin: bool,
    pub allow_camera_upload: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            allow_sync: true,
            allow_channels: true,
            allow_tuners: AllowTuners::None,
            allow_subtitle_admin: true,
            allow_camera_upload: false,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Filters {
    #[serde(rename = "filterMovies")]
    pub movies: SharingFilter,
    #[serde(rename = "filterMusic")]
    pub music: SharingFilter,
    #[serde(rename = "filterTelevision")]
    pub television: SharingFilter,
    #[serde(rename = "filterPhotos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<SharingFilter>,
    #[serde(rename = "filterAll", skip)]
    _all: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct SharingFilter {
    pub content_rating: Vec<String>,
    pub exclude_content_rating: Vec<String>,
    pub label: Vec<String>,
    pub exclude_label: Vec<String>,
}

impl<'de> Deserialize<'de> for SharingFilter {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = SharingFilter;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("valid plex filter string")
            }

            fn visit_str<E>(self, value: &str) -> StdResult<SharingFilter, E>
            where
                E: ::serde::de::Error,
            {
                let mut ret = SharingFilter::default();

                if value.is_empty() {
                    return Ok(ret);
                }

                for filter in value.split('|') {
                    let decoded_values =
                        serde_urlencoded::from_str::<Vec<(String, String)>>(filter);
                    if let Ok(decoded_values) = decoded_values {
                        for pairs in decoded_values {
                            match pairs.0.as_str() {
                                "contentRating" => {
                                    ret.content_rating =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "contentRating!" => {
                                    ret.exclude_content_rating =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "label" => {
                                    ret.label = pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                "label!" => {
                                    ret.exclude_label =
                                        pairs.1.split(',').map(|v| v.to_owned()).collect()
                                }
                                _ => {
                                    return Err(::serde::de::Error::invalid_value(
                                        ::serde::de::Unexpected::Str(value),
                                        &self,
                                    ));
                                }
                            }
                        }
                    } else {
                        return Err(::serde::de::Error::invalid_value(
                            ::serde::de::Unexpected::Str(value),
                            &self,
                        ));
                    }
                }

                Ok(ret)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl Serialize for SharingFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for SharingFilter {
    fn to_string(&self) -> String {
        let mut ret: Vec<String> = vec![];

        if !self.content_rating.is_empty() {
            ret.push(format!("contentRating={}", self.content_rating.join("%2C")))
        }

        if !self.exclude_content_rating.is_empty() {
            ret.push(format!(
                "contentRating!={}",
                self.exclude_content_rating.join("%2C")
            ))
        }

        if !self.label.is_empty() {
            ret.push(format!("label={}", self.label.join("%2C")))
        }

        if !self.exclude_label.is_empty() {
            ret.push(format!("label!={}", self.exclude_label.join("%2C")))
        }

        ret.join("|")
    }
}

pub enum ShareableServer<'a> {
    MachineIdentifier(&'a str),
    Server(&'a Server),
}

impl<'a> ShareableServer<'a> {
    pub fn get_id(&self) -> &str {
        match self {
            Self::MachineIdentifier(id) => id,
            Self::Server(srv) => srv.machine_identifier(),
        }
    }
}

pub enum ShareableLibrary<'a> {
    Library(&'a Library),
    LibraryId(&'a str),
}

impl<'a> ShareableLibrary<'a> {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Library(library) => library.id(),
            Self::LibraryId(id) => id,
        }
    }
}

pub enum User<'a> {
    Account(&'a MyPlex),
    UsernameOrEmail(&'a str),
}

impl<'a> User<'a> {
    pub fn get_identifier(&self) -> &str {
        match self {
            Self::Account(MyPlex {
                account: Some(account),
                ..
            }) => &account.email,
            Self::Account(_) => "",
            Self::UsernameOrEmail(u) => u,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ShareServerRequest {
    invited_email: String,
    settings: server::Settings,
    library_section_ids: Vec<u32>,
    machine_identifier: String,
}

impl<'a> Sharing<'a> {
    pub fn new(myplex: &'a MyPlex) -> Self {
        Sharing { myplex }
    }

    pub async fn invite<'b>(&self, user: User<'b>) -> Result<friend::Friend> {
        self.myplex
            .client()
            .post(format!(
                "{}?{}",
                MYPLEX_INVITES_INVITE,
                serde_urlencoded::to_string([("identifier", user.get_identifier())])?
            ))
            .json()
            .await
    }

    pub async fn share<'b, 'c, 'd>(
        &self,
        user: User<'b>,
        server: ShareableServer<'c>,
        sections: &[ShareableLibrary<'d>],
        permissions: Permissions,
        filters: Filters,
    ) -> Result<SharedServer> {
        let server_info = self.myplex.server_info(server.get_id()).await?;
        let sections: Vec<u32> = sections
            .iter()
            .filter_map(|s| s.get_id().parse().ok())
            .collect();
        let request = ShareServerRequest {
            invited_email: user.get_identifier().to_owned(),
            settings: server::Settings {
                allow_channels: permissions.allow_channels,
                allow_subtitle_admin: permissions.allow_subtitle_admin,
                allow_sync: permissions.allow_sync,
                allow_tuners: permissions.allow_tuners,
                allow_camera_upload: permissions.allow_camera_upload,
                filter_movies: Some(filters.movies),
                filter_television: Some(filters.television),
                filter_music: Some(filters.music),
                filter_photos: filters.photos,
                ..Default::default()
            },
            library_section_ids: server_info
                .library_sections
                .iter()
                .filter_map(|s| {
                    if sections.contains(&s.key) {
                        Some(s.id)
                    } else {
                        None
                    }
                })
                .collect(),
            machine_identifier: server.get_id().to_owned(),
        };

        self.myplex
            .client()
            .post(MYPLEX_INVITES_SHARED_SERVERS)
            .json_body(&request)?
            .json()
            .await
    }

    /// Returns a list of friends with the requested status, including managed users
    pub async fn friends(&self, status: InviteStatus) -> Result<Vec<friend::Friend>> {
        let mut friends: Vec<friend::Friend> = self
            .myplex
            .client()
            .get(format!("{}?includeSharedServers=true&includeSharedSources=true&includeSharingSettings=true&status={}", MYPLEX_INVITES_FRIENDS, status))
            .json()
            .await?;

        for friend in &mut friends {
            friend.client = Some(self.myplex.client().clone())
        }

        Ok(friends)
    }
}
