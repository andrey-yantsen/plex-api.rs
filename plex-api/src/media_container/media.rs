use crate::MediaType;
use serde_repr::Deserialize_repr;
use uuid::Uuid;

#[derive(Debug, Deserialize_repr, Clone)]
#[repr(u8)]
enum MediaStreamType {
    Video = 1,
    Audio = 2,
    Subtitles = 3,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaStream {
    id: u32,
    stream_type: MediaStreamType,
    default: Option<bool>,
    selected: Option<bool>,
    codec: String,
    index: u8,
    bitrate: u32,
    chroma_subsampling: Option<String>,
    chroma_location: Option<String>,
    closed_captions: Option<String>,
    coded_height: Option<String>,
    coded_width: Option<String>,
    color_primaries: Option<String>,
    color_space: Option<String>,
    color_range: Option<String>,
    color_trc: Option<String>,
    frame_rate: Option<f32>,
    height: Option<u16>,
    width: Option<u16>,
    level: Option<u16>,
    profile: Option<String>,
    ref_frames: Option<u64>,
    display_title: String,
    has_scaling_matrix: Option<bool>,
    scan_type: Option<String>,
    bit_depth: Option<u16>,
    sampling_rate: Option<u32>,
    channels: Option<u8>,
    audio_channel_layout: Option<String>,
    key: Option<String>,
    title: Option<String>,
    language: Option<String>,
    language_code: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_bool_from_anything"
    )]
    embedded_in_video: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaPart {
    id: u32,
    key: String,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    duration: chrono::Duration,
    file: String,
    size: u64,
    container: String,
    indexes: Option<String>,
    audio_profile: String,
    video_profile: String,
    #[serde(default, rename = "Stream")]
    streams: Vec<MediaStream>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaTag {
    id: Option<u32>,
    tag: String,
    filter: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    id: u32,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    duration: chrono::Duration,
    bitrate: u32,
    width: u16,
    height: u16,
    aspect_ratio: f32,
    audio_channels: u8,
    audio_codec: String,
    video_codec: String,
    video_resolution: String,
    container: String,
    video_frame_rate: String,
    audio_profile: String,
    video_profile: String,
    #[serde(rename = "Part")]
    parts: Option<Vec<MediaPart>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    allow_sync: bool,
    #[serde(rename = "librarySectionID")]
    library_section_id: u32,
    library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    library_section_uuid: Uuid,
    rating_key: String,
    key: String,
    skip_parent: Option<bool>,
    parent_rating_key: Option<String>,
    grandparent_rating_key: Option<String>,
    guid: Option<String>,
    parent_guid: Option<String>,
    grandparent_guid: Option<String>,
    #[serde(rename = "type")]
    media_type: MediaType,
    title: String,
    grandparent_key: Option<String>,
    parent_key: Option<String>,
    library_section_key: Option<String>,
    grandparent_title: Option<String>,
    parent_title: Option<String>,
    content_rating: Option<String>,
    summary: String,
    index: Option<u32>,
    parent_index: Option<u32>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_int_from_string"
    )]
    year: Option<u32>,
    thumb: String,
    art: Option<String>,
    parent_thumb: Option<String>,
    grandparent_thumb: Option<String>,
    grandparent_art: Option<String>,
    grandparent_theme: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_duration_from_seconds"
    )]
    duration: Option<chrono::Duration>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_date_from_iso"
    )]
    originally_available_at: Option<chrono::Date<chrono::Utc>>,
    #[serde(with = "chrono::serde::ts_seconds")]
    added_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, rename = "Media")]
    media: Vec<Media>,
    #[serde(rename = "Genre")]
    genre: Option<Vec<MediaTag>>,
    #[serde(rename = "Director")]
    director: Option<Vec<MediaTag>>,
    #[serde(rename = "Writer")]
    writer: Option<Vec<MediaTag>>,
    #[serde(rename = "Country")]
    country: Option<Vec<MediaTag>>,
    #[serde(rename = "Role")]
    role: Option<Vec<MediaTag>>,
    leaf_count: Option<u32>,
    viewed_leaf_count: Option<u32>,
    loudness_analysis_version: Option<String>,
    deep_analysis_version: Option<String>,
    studio: Option<String>,
    rating: Option<f32>,
    tagline: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_bool_from_anything"
    )]
    has_premium_primary_extra: Option<bool>,
    primary_extra_key: Option<String>,
    rating_image: Option<String>,
    parent_summary: Option<String>,
    parent_theme: Option<String>,
}
