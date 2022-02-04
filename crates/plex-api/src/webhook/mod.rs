//! This module contains a few structs that should help you handle the webhooks
//! received from a Plex server.
//!
//! The structs are implemented according to the [documentation](https://support.plex.tv/articles/115002267687-webhooks/),
//! please read it for further information.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Webhook {
    pub event: Event,
    pub user: bool,
    pub owner: bool,
    #[serde(rename = "Account")]
    pub account: Account,
    #[serde(rename = "Server")]
    pub server: Server,
    #[serde(rename = "Player")]
    pub player: Option<Player>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Account {
    pub id: u64,
    #[serde(with = "http_serde::uri")]
    pub thumb: http::Uri,
    pub title: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Server {
    pub title: String,
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub local: bool,
    pub public_address: String,
    pub title: String,
    pub uuid: String,
}

// TODO: combine with the regular metadata struct when we get one
#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub library_section_type: String,
    pub key: String,
    pub parent_key: String,
    pub grandparent_key: String,
    pub rating_key: String,
    pub parent_rating_key: String,
    pub grandparent_rating_key: String,
    pub guid: String,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: u32,
    pub r#type: String,
    pub title: String,
    pub parent_title: String,
    pub grandparent_title: String,
    pub summary: String,
    pub index: u32,
    pub parent_index: u32,
    pub rating_count: u64,
    pub thumb: String,
    pub parent_thumb: String,
    pub grandparent_thumb: String,
    pub art: String,
    pub grandparent_art: String,
    #[serde(with = "time::serde::timestamp")]
    pub added_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
}

/// Event type as described in the [Plex documentation](https://support.plex.tv/articles/115002267687-webhooks/).
#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    /// A new item is added that appears in the user’s On Deck. A poster is
    /// also attached to this event.
    #[serde(rename = "library.on.deck")]
    LibraryOnDeck,
    /// A new item is added to a library to which the user has access. A poster
    /// is also attached to this event.
    #[serde(rename = "library.new")]
    LibraryNew,

    /// Media playback pauses.
    #[serde(rename = "media.pause")]
    MediaPause,
    /// Media starts playing. An appropriate poster is attached.
    #[serde(rename = "media.play")]
    MediaPlay,
    /// Media is rated. A poster is also attached to this event.
    #[serde(rename = "media.rate")]
    MediaRate,
    /// Media playback resumes.
    #[serde(rename = "media.resume")]
    MediaResume,
    /// Media is viewed (played past the 90% mark).
    #[serde(rename = "media.scrobble")]
    MediaScrobble,
    /// Media playback stops.
    #[serde(rename = "media.stop")]
    MediaStop,

    /// A database backup is completed successfully via Scheduled Tasks.
    #[serde(rename = "admin.database.backup")]
    AdminDatabaseBackup,
    /// Corruption is detected in the server database.
    #[serde(rename = "admin.database.corrupted")]
    AdminDatabaseCorrupted,
    /// A device accesses the owner’s server for any reason, which may come
    /// from background connection testing and doesn’t necessarily indicate
    /// active browsing or playback.
    #[serde(rename = "device.new")]
    DeviceNew,
    /// Playback is started by a shared user for the server. A poster is also
    /// attached to this event.
    #[serde(rename = "playback.started")]
    PlaybackStarted,

    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod test {
    #[plex_api_test_helper::offline_test]
    fn check_parsing() {
        serde_json::from_str::<super::Webhook>(WEBHOOK_JSON).expect("failed to parse webhook");
    }

    const WEBHOOK_JSON: &str = r#"{  
        "event": "media.play",
        "user": true,
        "owner": true,
        "Account": {
           "id": 1,
           "thumb": "https://plex.tv/users/1022b120ffbaa/avatar?c=1465525047",
           "title": "elan"
        },
        "Server": {
           "title": "Office",
           "uuid": "54664a3d8acc39983675640ec9ce00b70af9cc36"
        },
        "Player": {
           "local": true,
           "publicAddress": "200.200.200.200",
           "title": "Plex Web (Safari)",
           "uuid": "r6yfkdnfggbh2bdnvkffwbms"
        },
        "Metadata": {
           "librarySectionType": "artist",
           "ratingKey": "1936545",
           "key": "/library/metadata/1936545",
           "parentRatingKey": "1936544",
           "grandparentRatingKey": "1936543",
           "guid": "com.plexapp.agents.plexmusic://gracenote/track/7572499-91016293BE6BF7F1AB2F848F736E74E5/7572500-3CBAE310D4F3E66C285E104A1458B272?lang=en",
           "librarySectionID": 1224,
           "type": "track",
           "title": "Love The One You're With",
           "grandparentKey": "/library/metadata/1936543",
           "parentKey": "/library/metadata/1936544",
           "grandparentTitle": "Stephen Stills",
           "parentTitle": "Stephen Stills",
           "summary": "",
           "index": 1,
           "parentIndex": 1,
           "ratingCount": 6794,
           "thumb": "/library/metadata/1936544/thumb/1432897518",
           "art": "/library/metadata/1936543/art/1485951497",
           "parentThumb": "/library/metadata/1936544/thumb/1432897518",
           "grandparentThumb": "/library/metadata/1936543/thumb/1485951497",
           "grandparentArt": "/library/metadata/1936543/art/1485951497",
           "addedAt": 1000396126,
           "updatedAt": 1432897518
        }
     }
"#;
}
