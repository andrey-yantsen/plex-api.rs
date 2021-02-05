use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr, Clone)]
#[repr(u8)]
enum MediaStreamType {
    Video = 1,
    Audio = 2,
    Subtitles = 3,
    Lyrics = 4,
}

#[derive(Debug, Clone)]
pub enum MediaStream {
    Video(VideoStream),
    Audio(AudioStream),
    Subtitles(SubtitlesStream),
    Lyrics(LyricsStream),
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
struct MediaStreamStruct {
    id: u32,
    stream_type: MediaStreamType,
    default: Option<bool>,
    selected: Option<bool>,
    codec: String,
    index: u8,
    bitrate: Option<u32>,
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
    extended_display_title: Option<String>,
    album_gain: Option<f32>,
    album_peak: Option<f32>,
    album_range: Option<f32>,
    gain: Option<f32>,
    loudness: Option<f32>,
    lra: Option<f32>,
    peak: Option<f32>,
    format: Option<String>,
    provider: Option<String>,
}

macro_rules! media_stream_enum {
    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty,)+
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            id: u32,
            stream_type: MediaStreamType,
            codec: String,
            index: u8,
            display_title: String,
            $($field_name: $field_type,)+
        }

        impl From<MediaStreamStruct> for $name {
            fn from(stream: MediaStreamStruct) -> Self {
                $name {
                    id: stream.id,
                    stream_type: stream.stream_type,
                    codec: stream.codec,
                    index: stream.index,
                    display_title: stream.display_title,
                    $($field_name: <$field_type>::convert(stringify!($field_name), stream.$field_name)),+
                }
            }
        }
    }
}

trait InternalTypesConverter<T>: Sized {
    fn convert(field: &str, _: T) -> Self;
}

impl<T> InternalTypesConverter<Option<T>> for Option<T> {
    fn convert(_field: &str, input: Option<T>) -> Self {
        input
    }
}

impl<T: Default> InternalTypesConverter<Option<T>> for T {
    fn convert(field: &str, input: Option<T>) -> Self {
        if let Some(value) = input {
            value
        } else {
            warn!("Error while processing field {}", field);
            Default::default()
        }
    }
}

media_stream_enum! {
    pub struct VideoStream {
        default: bool,
        bitrate: u32,
        chroma_subsampling: String,
        chroma_location: String,
        closed_captions: String,
        coded_height: String,
        coded_width: String,
        color_primaries: String,
        color_space: String,
        color_range: String,
        color_trc: String,
        frame_rate: f32,
        height: u16,
        width: u16,
        level: u16,
        profile: String,
        ref_frames: u64,
        has_scaling_matrix: bool,
        scan_type: String,
        bit_depth: u16,
    }
}

media_stream_enum! {
    pub struct AudioStream {
        default: Option<bool>,
        selected: bool,
        bitrate: u32,
        profile: String,
        sampling_rate: u32,
        channels: u8,
        audio_channel_layout: String,
        album_gain: Option<f32>,
        album_peak: Option<f32>,
        album_range: Option<f32>,
        bit_depth: Option<u16>,
        extended_display_title: Option<String>,
        gain: Option<f32>,
        loudness: Option<f32>,
        lra: Option<f32>,
        peak: Option<f32>,
    }
}

media_stream_enum! {
    pub struct SubtitlesStream {
        selected: bool,
        bitrate: Option<u32>,
        embedded_in_video: bool,
    }
}

media_stream_enum! {
    pub struct LyricsStream {
        extended_display_title: Option<String>,
        format: Option<String>,
        provider: Option<String>,
    }
}

impl MediaStream {
    fn new(stream: MediaStreamStruct) -> Self {
        match stream.stream_type {
            MediaStreamType::Video => MediaStream::Video(VideoStream::from(stream)),
            MediaStreamType::Audio => MediaStream::Audio(AudioStream::from(stream)),
            MediaStreamType::Subtitles => MediaStream::Subtitles(SubtitlesStream::from(stream)),
            MediaStreamType::Lyrics => MediaStream::Lyrics(LyricsStream::from(stream)),
        }
    }
}

impl<'de> Deserialize<'de> for MediaStream {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stream = MediaStreamStruct::deserialize(d)?;
        Ok(MediaStream::new(stream))
    }
}
