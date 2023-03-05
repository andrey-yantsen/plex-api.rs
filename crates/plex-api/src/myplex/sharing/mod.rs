use crate::media_container::users::AllowTuners;
use crate::HttpClient;
use crate::Library;
use crate::Result;
use crate::Server;
use ::serde::de::Deserializer;
use ::serde::de::Visitor;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;
use ::std::result::Result as StdResult;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sharing {
    client: Arc<HttpClient>,
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

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Filters {
    #[serde(rename = "filterMovies")]
    pub movies: SharingFilter,
    #[serde(rename = "filterMusic")]
    pub music: SharingFilter,
    #[serde(rename = "filterTelevision")]
    pub television: SharingFilter,
    #[serde(rename = "filterPhotos", skip)]
    _photos: Option<String>,
    #[serde(rename = "filterAll", skip)]
    _all: Option<String>,
}

#[derive(Default, Debug)]
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

                for filter in value.split('|') {
                    let pairs: Vec<&str> = filter.split('=').collect();

                    if pairs.len() != 2 {
                        return Err(::serde::de::Error::invalid_value(
                            ::serde::de::Unexpected::Str(value),
                            &self,
                        ));
                    }

                    match pairs[0] {
                        "contentRating" => {
                            ret.content_rating =
                                pairs[1].split("%2C").map(|v| v.to_owned()).collect()
                        }
                        "contentRating!" => {
                            ret.exclude_content_rating =
                                pairs[1].split("%2C").map(|v| v.to_owned()).collect()
                        }
                        "label" => {
                            ret.label = pairs[1].split("%2C").map(|v| v.to_owned()).collect()
                        }
                        "label!" => {
                            ret.exclude_label =
                                pairs[1].split("%2C").map(|v| v.to_owned()).collect()
                        }
                        _ => {
                            return Err(::serde::de::Error::invalid_value(
                                ::serde::de::Unexpected::Str(value),
                                &self,
                            ));
                        }
                    }
                }

                Ok(ret)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl ::serde::ser::Serialize for SharingFilter {
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
    Server(Server),
}

pub enum ShareableLibrary {
    Library(Library),
    LibraryId(u16),
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InviteStatus {
    PendingSent,
    Accepted,
    PendingReceived,
}

impl Sharing {
    pub fn new(client: Arc<HttpClient>) -> Self {
        Sharing { client }
    }

    pub async fn invite(&self, username_or_email: &str) -> Result<&str> {
        Ok("")
    }

    pub async fn share<'a, 'b>(
        &self,
        username_or_email: &'a str,
        server: ShareableServer<'b>,
        sections: Vec<ShareableLibrary>,
        permissions: Permissions,
        filters: Filters,
    ) -> Result<&str> {
        Ok("")
    }

    pub async fn update_share(
        &self,
        invite_id: u16,
        sections: Option<Vec<ShareableLibrary>>,
        permissions: Option<Permissions>,
        filters: Option<Filters>,
    ) -> Result<&str> {
        Ok("")
    }

    pub async fn delete(&self, invite_id: u16) -> Result<&str> {
        Ok("")
    }

    pub async fn pending_invites(&self) -> Result<Vec<&str>> {
        Ok(vec![])
    }
}
