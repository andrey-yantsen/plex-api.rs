mod filter;
mod friend;
mod server;
mod shareables;

use crate::{
    media_container::users::AllowTuners,
    url::{MYPLEX_INVITES_FRIENDS, MYPLEX_INVITES_INVITE, MYPLEX_INVITES_SHARED_SERVERS},
    MyPlex, Result,
};
use serde::{Deserialize, Serialize};

pub use filter::SharingFilter;
pub use friend::{Friend, InviteStatus};
pub use server::SharedServer;
pub use shareables::*;

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

    #[tracing::instrument(level = "debug", skip_all, fields(identifier = user.id()))]
    pub async fn invite<'b>(&self, user: User<'b>) -> Result<friend::Friend> {
        self.myplex
            .client()
            .post(format!(
                "{}?{}",
                MYPLEX_INVITES_INVITE,
                serde_urlencoded::to_string([("identifier", user.id())])?
            ))
            .json()
            .await
    }

    #[tracing::instrument(level = "debug", skip_all, fields(user_identifier = user.id(), machine_identifier = server.id()))]
    pub async fn share<'b, 'c, 'd>(
        &self,
        user: User<'b>,
        server: ShareableServer<'c>,
        sections: &[ShareableLibrary<'d>],
        permissions: Permissions,
        filters: Filters,
    ) -> Result<SharedServer> {
        let server_info = self.myplex.server_info(server.id()).await?;
        let sections: Vec<u32> = sections
            .iter()
            .filter_map(|s| s.id().parse().ok())
            .collect();
        let request = ShareServerRequest {
            invited_email: user.id().to_owned(),
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
            machine_identifier: server.id().to_owned(),
        };

        self.myplex
            .client()
            .post(MYPLEX_INVITES_SHARED_SERVERS)
            .json_body(&request)?
            .json()
            .await
    }

    /// Returns a list of friends with the requested status, including managed users
    #[tracing::instrument(level = "debug", skip(self))]
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
