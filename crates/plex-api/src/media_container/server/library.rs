use crate::media_container::MediaContainer;
use monostate::MustBe;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::{
    deserialize_bool_from_anything, deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use serde_json::Value;
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use time::{Date, OffsetDateTime};

fn optional_boolish<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_bool_from_anything(deserializer)?))
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Copy,
    Transcode,
    Ignore,
    DirectPlay,
    Burn,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_display_from_serialize!(Decision);

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    // HTTP file download
    Http,
    // HTTP Live Streaming
    Hls,
    // Dynamic Adaptive Streaming over HTTP
    Dash,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_display_from_serialize!(Protocol);

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
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
derive_display_from_serialize!(AudioCodec);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
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
derive_display_from_serialize!(VideoCodec);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum LyricCodec {
    Lrc,
    Txt,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(LyricCodec);
derive_display_from_serialize!(LyricCodec);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubtitleCodec {
    Ass,
    Pgs,
    Subrip,
    Srt,
    DvdSubtitle,
    MovText,
    Vtt,
    DvbSubtitle,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(SubtitleCodec);
derive_display_from_serialize!(SubtitleCodec);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
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
    Ogg,
    Wav,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(ContainerFormat);
derive_display_from_serialize!(ContainerFormat);

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct VideoStream {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub stream_type: MustBe!(1),
    pub index: Option<u32>,
    pub codec: VideoCodec,
    pub default: Option<bool>,
    pub selected: Option<bool>,
    pub title: Option<String>,
    pub display_title: String,
    pub extended_display_title: String,

    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, u32>>")]
    pub required_bandwidths: Option<Vec<u32>>,
    pub decision: Option<Decision>,
    pub location: Option<String>,

    pub height: u32,
    pub width: u32,
    pub bit_depth: Option<u8>,
    pub bitrate: u32,
    pub chroma_location: Option<String>,
    pub chroma_subsampling: Option<String>,
    pub coded_height: Option<u32>,
    pub coded_width: Option<u32>,
    pub color_primaries: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub color_trc: Option<String>,
    pub frame_rate: f32,
    pub has_scaling_matrix: Option<bool>,
    pub level: Option<u32>,
    pub profile: Option<String>,
    pub ref_frames: Option<u32>,
    pub scan_type: Option<String>,
    #[serde(rename = "codecID")]
    pub codec_id: Option<String>,
    pub stream_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct AudioStream {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub stream_type: MustBe!(2),
    pub index: Option<u32>,
    pub codec: AudioCodec,
    pub default: Option<bool>,
    pub selected: Option<bool>,
    pub title: Option<String>,
    pub display_title: String,
    pub extended_display_title: String,

    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, u32>>")]
    pub required_bandwidths: Option<Vec<u32>>,
    pub decision: Option<Decision>,
    pub location: Option<String>,

    pub channels: u32,
    pub audio_channel_layout: Option<String>,
    pub profile: Option<String>,
    pub sampling_rate: Option<u32>,
    pub bitrate: Option<u32>,
    pub bitrate_mode: Option<String>,
    pub language: Option<String>,
    pub language_code: Option<String>,
    pub language_tag: Option<String>,
    pub peak: Option<String>,
    pub gain: Option<String>,
    pub album_peak: Option<String>,
    pub album_gain: Option<String>,
    pub album_range: Option<String>,
    pub lra: Option<String>,
    pub loudness: Option<String>,
    pub stream_identifier: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SubtitleStream {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub stream_type: MustBe!(3),
    pub index: Option<u32>,
    pub codec: SubtitleCodec,
    pub default: Option<bool>,
    pub selected: Option<bool>,
    pub title: Option<String>,
    pub display_title: String,
    pub extended_display_title: String,

    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, u32>>")]
    pub required_bandwidths: Option<Vec<u32>>,
    pub decision: Option<Decision>,
    pub location: Option<String>,

    pub key: Option<String>,
    pub format: Option<String>,
    pub file: Option<String>,
    pub bitrate: Option<u32>,
    pub hearing_impaired: Option<bool>,
    pub language: Option<String>,
    pub language_code: Option<String>,
    pub language_tag: Option<String>,
    pub ignore: Option<String>,
    pub burn: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct LyricStream {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub stream_type: MustBe!(4),
    pub index: Option<u32>,
    pub codec: LyricCodec,
    pub default: Option<bool>,
    pub selected: Option<bool>,
    pub title: Option<String>,
    pub display_title: String,
    pub extended_display_title: String,

    pub key: Option<String>,
    pub format: Option<String>,
    pub timed: Option<String>,
    pub min_lines: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(not(feature = "tests_deny_unknown_fields"), serde(untagged))]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(try_from = "Value"))]
pub enum Stream {
    Video(VideoStream),
    Audio(AudioStream),
    Subtitle(SubtitleStream),
    Lyric(LyricStream),
    Unknown(Value),
}

// This generates much saner errors in tests than an untagged enum.
#[cfg(feature = "tests_deny_unknown_fields")]
impl TryFrom<Value> for Stream {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let stream_type = match &value {
            Value::Object(o) => {
                if let Some(Value::Number(n)) = o.get("streamType") {
                    if let Some(v) = n.as_u64() {
                        v
                    } else {
                        return Err(format!(
                            "Failed to decode Stream. Unexpected streamType `{n}`"
                        ));
                    }
                } else {
                    return Err("Failed to decode Stream. Missing streamType property.".to_string());
                }
            }
            _ => return Err("Failed to decode Stream. Data was not an object.".to_string()),
        };

        if stream_type == 1 {
            let s: VideoStream = serde_json::from_value(value)
                .map_err(|e| format!("Failed to decode video stream: {e}"))?;
            Ok(Self::Video(s))
        } else if stream_type == 2 {
            let s: AudioStream = serde_json::from_value(value)
                .map_err(|e| format!("Failed to decode audio stream: {e}"))?;
            Ok(Self::Audio(s))
        } else if stream_type == 3 {
            let s: SubtitleStream = serde_json::from_value(value)
                .map_err(|e| format!("Failed to decode subtitle stream: {e}"))?;
            Ok(Self::Subtitle(s))
        } else if stream_type == 4 {
            let s: LyricStream = serde_json::from_value(value)
                .map_err(|e| format!("Failed to decode lyric stream: {e}"))?;
            Ok(Self::Lyric(s))
        } else if !cfg!(feature = "tests_deny_unknown_fields") {
            Ok(Self::Unknown(value))
        } else {
            Err(format!(
                "Failed to decode Stream. Unexpected streamType `{stream_type}`"
            ))
        }
    }
}

#[serde_as]
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
    pub protocol: Option<Protocol>,
    pub selected: Option<bool>,
    pub decision: Option<Decision>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub packet_length: Option<u64>,
    pub has_thumbnail: Option<String>,
    #[serde(rename = "has64bitOffsets")]
    pub has_64bit_offsets: Option<bool>,
    #[serde(default, deserialize_with = "optional_boolish")]
    pub optimized_for_streaming: Option<bool>,
    pub has_chapter_text_stream: Option<bool>,
    pub deep_analysis_version: Option<String>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, u32>>")]
    pub required_bandwidths: Option<Vec<u32>>,
    pub bitrate: Option<u32>,
    #[serde(rename = "Stream")]
    pub streams: Option<Vec<Stream>>,
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
    pub protocol: Option<Protocol>,
    pub audio_codec: Option<AudioCodec>,
    pub video_codec: Option<VideoCodec>,
    pub video_resolution: Option<String>,
    pub container: Option<ContainerFormat>,
    pub video_frame_rate: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    pub selected: Option<bool>,
    #[serde(rename = "Part")]
    pub parts: Vec<Part>,
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
pub struct Guid {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Tag {
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u32>,
    pub tag: String,
    pub filter: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub count: u32,
    pub image: String,
    #[serde(rename = "type")]
    pub rating_type: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub value: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u32>,
    pub tag: String,
    pub tag_key: Option<String>,
    pub filter: Option<String>,
    pub role: Option<String>,
    pub thumb: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub count: Option<u32>,
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
    #[serde(rename = "album")]
    MusicAlbum,
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
#[serde(rename_all = "lowercase")]
pub enum PlaylistType {
    Video,
    Audio,
    Photo,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

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
    pub subtype: Option<MetadataType>,
    pub playlist_type: Option<PlaylistType>,
    pub smart: Option<bool>,
    #[serde(default, deserialize_with = "optional_boolish")]
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
    #[serde(default, deserialize_with = "optional_boolish")]
    pub has_premium_lyrics: Option<bool>,
    pub music_analysis_version: Option<String>,

    #[serde(rename = "librarySectionID")]
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub library_section_id: Option<u32>,
    pub library_section_title: Option<String>,
    pub library_section_key: Option<String>,

    #[serde(flatten)]
    pub parent: ParentMetadata,
    #[serde(flatten)]
    pub grand_parent: GrandParentMetadata,

    #[serde(default, rename = "Guid")]
    pub guids: Vec<Guid>,
    #[serde(default, rename = "Collection")]
    pub collections: Vec<Tag>,
    #[serde(default, rename = "Similar")]
    pub similar: Vec<Tag>,
    #[serde(default, rename = "Genre")]
    pub genres: Vec<Tag>,
    #[serde(default, rename = "Director")]
    pub directors: Vec<Tag>,
    #[serde(default, rename = "Producer")]
    pub producers: Vec<Tag>,
    #[serde(default, rename = "Writer")]
    pub writers: Vec<Tag>,
    #[serde(default, rename = "Country")]
    pub countries: Vec<Tag>,
    #[serde(default, rename = "Rating")]
    pub ratings: Vec<Rating>,
    #[serde(default, rename = "Role")]
    pub roles: Vec<Role>,
    #[serde(default, rename = "Location")]
    pub locations: Vec<Location>,
    #[serde(default, rename = "Field")]
    pub fields: Vec<Field>,
    #[serde(default, rename = "Mood")]
    pub moods: Vec<Tag>,

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

    #[serde(default, deserialize_with = "optional_boolish")]
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
    pub playlist_type: Option<PlaylistType>,

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
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    Unknown(Value),
}
