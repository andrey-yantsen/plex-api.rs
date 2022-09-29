use crate::media_container::MediaContainer;
use monostate::MustBe;
use serde::{Deserialize, Deserializer};
use serde_aux::prelude::{
    deserialize_bool_from_anything, deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use serde_json::Value;
use serde_plain::derive_fromstr_from_deserialize;
use time::{Date, OffsetDateTime};

fn optional_boolish<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_bool_from_anything(deserializer)?))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChapterSource {
    Media,
    Mixed,
    Agent,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(ChapterSource);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AudioCodec {
    Aac,
    Ac3,
    Dca,
    DcaMa,
    Eac3,
    Mp2,
    Mp3,
    Opus,
    Pcm,
    Vorbis,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(AudioCodec);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VideoCodec {
    H264,
    Hevc,
    Mpeg1Video,
    Mpeg2Video,
    Mpeg4,
    Msmpeg4v3,
    Vc1,
    Vp8,
    Vp9,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(VideoCodec);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerFormat {
    Aac,
    Avi,
    Jpeg,
    M4v,
    Mkv,
    Mp3,
    Mp4,
    Mpeg,
    MpegTs,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(ContainerFormat);

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub key: Option<String>,
    pub duration: Option<u64>,
    pub file: Option<String>,
    pub size: Option<u64>,
    pub container: Option<ContainerFormat>,
    pub indexes: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    pub packet_length: Option<u64>,
    pub has_thumbnail: Option<String>,
    #[serde(rename = "has64bitOffsets")]
    pub has_64bit_offsets: Option<bool>,
    #[serde(default, deserialize_with = "optional_boolish")]
    pub optimized_for_streaming: Option<bool>,
    pub has_chapter_text_stream: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub duration: Option<u64>,
    pub bitrate: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub aspect_ratio: Option<f32>,
    pub audio_channels: Option<u8>,
    pub audio_codec: Option<AudioCodec>,
    pub video_codec: Option<VideoCodec>,
    pub video_resolution: Option<String>,
    pub container: Option<ContainerFormat>,
    pub video_frame_rate: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    #[serde(rename = "Part")]
    pub part: Vec<Part>,
    #[serde(rename = "has64bitOffsets")]
    pub has_64bit_offsets: Option<bool>,
    #[serde(default, deserialize_with = "optional_boolish")]
    pub optimized_for_streaming: Option<bool>,
    pub display_offset: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Field {
    pub locked: bool,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Location {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Tag {
    pub id: Option<u32>,
    pub tag: String,
    pub filter: Option<String>,
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Role {
    pub id: Option<u32>,
    pub tag: String,
    pub filter: Option<String>,
    pub role: Option<String>,
    pub thumb: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ParentMetadata {
    pub parent_key: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub parent_rating_key: Option<u32>,
    pub parent_guid: Option<String>,

    pub parent_title: Option<String>,
    pub parent_studio: Option<String>,
    pub parent_year: Option<u32>,
    pub parent_content_rating: Option<String>,
    pub parent_index: Option<u32>,

    pub parent_thumb: Option<String>,
    pub parent_art: Option<String>,
    pub parent_theme: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GrandParentMetadata {
    pub grandparent_key: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub grandparent_rating_key: Option<u32>,
    pub grandparent_guid: Option<String>,

    pub grandparent_title: Option<String>,
    pub grandparent_studio: Option<String>,
    pub grandparent_year: Option<u32>,
    pub grandparent_content_rating: Option<String>,
    pub grandparent_index: Option<u32>,

    pub grandparent_thumb: Option<String>,
    pub grandparent_art: Option<String>,
    pub grandparent_theme: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetadataType {
    Movie,
    Episode,
    Photo,
    Show,
    Artist,
    Album,
    Collection,
    Season,
    Track,
    Playlist,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(MetadataType);

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub key: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub rating_key: u32,
    pub guid: String,

    #[serde(rename = "type")]
    pub metadata_type: Option<MetadataType>,
    pub subtype: Option<String>,
    pub playlist_type: Option<String>,
    pub smart: Option<bool>,
    pub allow_sync: Option<bool>,

    pub title: String,
    pub title_sort: Option<String>,
    pub original_title: Option<String>,
    pub studio: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub year: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub min_year: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub max_year: Option<u32>,
    pub content_rating: Option<String>,
    pub summary: Option<String>,
    pub rating: Option<f32>,
    pub rating_count: Option<u32>,
    pub rating_image: Option<String>,
    pub audience_rating: Option<f32>,
    pub audience_rating_image: Option<String>,
    pub tagline: Option<String>,
    pub duration: Option<u64>,
    pub originally_available_at: Option<Date>,

    pub thumb: Option<String>,
    pub art: Option<String>,
    pub theme: Option<String>,
    pub composite: Option<String>,
    pub banner: Option<String>,
    pub icon: Option<String>,

    pub index: Option<u32>,
    #[serde(rename = "playlistItemID")]
    pub playlist_item_id: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub child_count: Option<u32>,
    pub leaf_count: Option<u32>,
    pub viewed_leaf_count: Option<u32>,
    pub skip_children: Option<bool>,

    pub view_count: Option<u64>,
    pub skip_count: Option<u64>,
    #[serde(default, with = "time::serde::timestamp::option")]
    pub last_viewed_at: Option<OffsetDateTime>,

    #[serde(rename = "createdAtTZOffset")]
    pub created_at_tz_offset: Option<String>,
    pub created_at_accuracy: Option<String>,
    #[serde(with = "time::serde::timestamp")]
    pub added_at: OffsetDateTime,
    #[serde(default, with = "time::serde::timestamp::option")]
    pub deleted_at: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::timestamp::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub loudness_analysis_version: Option<u32>,
    #[serde(default, deserialize_with = "optional_boolish")]
    pub has_premium_primary_extra: Option<bool>,
    pub view_offset: Option<u64>,
    pub chapter_source: Option<ChapterSource>,
    pub primary_extra_key: Option<String>,

    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<u32>,
    pub library_section_title: Option<String>,
    pub library_section_key: Option<String>,

    #[serde(flatten)]
    pub parent: ParentMetadata,
    #[serde(flatten)]
    pub grand_parent: GrandParentMetadata,

    #[serde(default, rename = "Collection")]
    pub collections: Vec<Tag>,
    #[serde(default, rename = "Similar")]
    pub similar: Vec<Tag>,
    #[serde(default, rename = "Genre")]
    pub genres: Vec<Tag>,
    #[serde(default, rename = "Director")]
    pub directors: Vec<Tag>,
    #[serde(default, rename = "Writer")]
    pub writers: Vec<Tag>,
    #[serde(default, rename = "Country")]
    pub countries: Vec<Tag>,
    #[serde(default, rename = "Role")]
    pub roles: Vec<Role>,
    #[serde(default, rename = "Location")]
    pub locations: Vec<Location>,
    #[serde(default, rename = "Field")]
    pub fields: Vec<Field>,

    #[serde(rename = "Media")]
    pub media: Option<Vec<Media>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MetadataMediaContainer {
    pub key: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub rating_key: Option<u32>,

    pub title: Option<String>,
    pub title1: Option<String>,
    pub title2: Option<String>,
    pub summary: Option<String>,
    pub duration: Option<u64>,

    pub allow_sync: Option<bool>,
    #[serde(rename = "nocache")]
    pub no_cache: Option<bool>,
    pub sort_asc: Option<bool>,
    pub smart: Option<bool>,

    pub thumb: Option<String>,
    pub art: Option<String>,
    pub theme: Option<String>,
    pub composite: Option<String>,
    pub banner: Option<String>,

    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<u32>,
    pub library_section_title: Option<String>,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: Option<String>,

    #[serde(flatten)]
    pub parent: ParentMetadata,
    #[serde(flatten)]
    pub grand_parent: GrandParentMetadata,
    #[serde(flatten)]
    pub media_container: MediaContainer,

    pub media_tag_prefix: Option<String>,
    #[serde(default, with = "time::serde::timestamp::option")]
    pub media_tag_version: Option<OffsetDateTime>,
    pub view_group: Option<String>,
    pub view_mode: Option<u32>,
    pub leaf_count: Option<u32>,
    pub playlist_type: Option<String>,

    #[serde(default, rename = "Directory")]
    pub directories: Vec<Value>,
    #[serde(default, rename = "Metadata")]
    pub metadata: Vec<Metadata>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PivotType {
    Hub,
    List,
    View,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(PivotType);

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Pivot {
    pub context: String,
    pub id: String,
    pub key: String,
    pub symbol: String,
    pub title: String,
    #[serde(rename = "type")]
    pub pivot_type: PivotType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LibraryType {
    Movie,
    Show,
    Artist,
    Photo,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(LibraryType);

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ServerLibrary {
    #[serde(rename = "type")]
    pub library_type: LibraryType,
    #[serde(rename = "Pivot")]
    pub pivots: Vec<Pivot>,
    pub agent: String,
    pub hub_key: String,
    pub id: String,
    pub key: String,
    pub subtype: Option<String>,
    pub language: String,
    pub refreshing: bool,
    #[serde(with = "time::serde::timestamp")]
    pub scanned_at: OffsetDateTime,
    pub scanner: String,
    pub title: String,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
    pub uuid: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ServerPlaylists {
    #[serde(rename = "type")]
    _type: MustBe!("playlist"),
    #[serde(rename = "Pivot")]
    pub pivots: Vec<Pivot>,
    pub id: String,
    pub key: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ServerHome {
    pub hub_key: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ContentDirectory {
    Playlists(ServerPlaylists),
    #[serde(rename_all = "camelCase")]
    Media(Box<ServerLibrary>),
    #[serde(rename_all = "camelCase")]
    Home(ServerHome),
}
