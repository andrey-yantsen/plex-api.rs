use serde::{Deserialize, Deserializer};
use serde_plain::derive_fromstr_from_deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum MetadataType {
    Movie,
    Episode,
    Photo,
    Show,
    Artist,
    MusicAlbum,
    Collection(CollectionMetadataSubtype),
    Season,
    Track,
    Playlist(PlaylistMetadataType),
    Clip(ClipMetadataSubtype),
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    Unknown,
}

pub(crate) fn deserialize_option_metadata_type<'de, D>(
    deserializer: D,
) -> Result<Option<MetadataType>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Helper {
        r#type: Option<String>,
        subtype: Option<String>,
        playlist_type: Option<String>,
        extra_type: Option<ExtraType>,
    }

    #[derive(Debug, Deserialize_repr, Clone, Serialize_repr)]
    #[repr(u8)]
    enum ExtraType {
        Trailer = 1,
        DeletedScene = 2,
        Interview = 3,
        MusicVideo = 4,
        BehindTheScenes = 5,
        Scene = 6,
        LiveMusicVideo = 7,
        LyricMusicVideo = 8,
        Contert = 9,
        Featurette = 10,
        Short = 11,
        Other = 12,
        #[cfg(not(feature = "tests_deny_unknown_fields"))]
        #[serde(other)]
        Unknown,
    }

    let m = Helper::deserialize(deserializer)?;

    match m.r#type {
        Some(metadata_type) => Ok(Some(match metadata_type.as_str() {
            "movie" => MetadataType::Movie,
            "episode" => MetadataType::Episode,
            "photo" => MetadataType::Photo,
            "show" => MetadataType::Show,
            "artist" => MetadataType::Artist,
            "album" => MetadataType::MusicAlbum,
            "season" => MetadataType::Season,
            "track" => MetadataType::Track,
            "collection" => match m.subtype {
                Some(subtype) => {
                    MetadataType::Collection(match CollectionMetadataSubtype::from_str(&subtype) {
                        Ok(subtype) => subtype,
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        Err(_) => CollectionMetadataSubtype::Unknown,
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        Err(_) => return Err(serde::de::Error::unknown_variant("unexpected subtype for collection", &["movie", "show"])),
                    })
                }
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
                _ => MetadataType::Collection(CollectionMetadataSubtype::Unknown),
                #[cfg(feature = "tests_deny_unknown_fields")]
                _ => return Err(serde::de::Error::missing_field(
                    "metadata with `collection` type expects a subtype field present, which is missing",
                )),
            },
            "clip" => match m.subtype {
                Some(subtype) => {
                    MetadataType::Clip(match ClipMetadataSubtype::from_str(&subtype) {
                        Ok(subtype) => subtype,
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        Err(_) => ClipMetadataSubtype::Unknown,
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        Err(_) => return Err(serde::de::Error::unknown_variant(&subtype, &["trailer"])),
                    })
                }
                None if m.extra_type.is_some() => MetadataType::Clip(match m.extra_type {
                    Some(subtype) => match subtype {
                        ExtraType::Trailer => ClipMetadataSubtype::Trailer,
                        ExtraType::DeletedScene => ClipMetadataSubtype::DeletedScene,
                        ExtraType::Interview => ClipMetadataSubtype::Interview,
                        ExtraType::MusicVideo => ClipMetadataSubtype::MusicVideo,
                        ExtraType::BehindTheScenes => ClipMetadataSubtype::BehindTheScenes,
                        ExtraType::Scene => ClipMetadataSubtype::SceneOrSample,
                        ExtraType::LiveMusicVideo => ClipMetadataSubtype::LiveMusicVideo,
                        ExtraType::LyricMusicVideo => ClipMetadataSubtype::LyricMusicVideo,
                        ExtraType::Contert => ClipMetadataSubtype::Concert,
                        ExtraType::Featurette => ClipMetadataSubtype::Featurette,
                        ExtraType::Short => ClipMetadataSubtype::Short,
                        ExtraType::Other => ClipMetadataSubtype::Other,
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        ExtraType::Unknown => ClipMetadataSubtype::Unknown,
                    },
                    #[cfg(not(feature = "tests_deny_unknown_fields"))]
                    _ => ClipMetadataSubtype::Unknown,
                    #[cfg(feature = "tests_deny_unknown_fields")]
                    _ => return Err(serde::de::Error::unknown_variant("empty extra_type", &["trailer"])),
                }),
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
                _ => MetadataType::Clip(ClipMetadataSubtype::Unknown),
                #[cfg(feature = "tests_deny_unknown_fields")]
                _ => return Err(serde::de::Error::missing_field(
                    "metadata with `clip` type expects a subtype field present, which is missing",
                )),
            },
            "playlist" => match m.playlist_type {
                Some(subtype) => {
                    MetadataType::Playlist(match PlaylistMetadataType::from_str(&subtype) {
                        Ok(subtype) => subtype,
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        Err(_) => PlaylistMetadataType::Unknown,
                        #[cfg(feature = "tests_deny_unknown_fields")]
                        Err(_) => return Err(serde::de::Error::unknown_variant(&subtype, &["video", "audio", "photo"])),
                    })
                }
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
                _ => MetadataType::Playlist(PlaylistMetadataType::Unknown),
                #[cfg(feature = "tests_deny_unknown_fields")]
                _ => return Err(serde::de::Error::missing_field(
                    "metadata with `playlist` type expects a playlist_type field present, which is missing",
                )),
            },
            #[cfg(not(feature = "tests_deny_unknown_fields"))]
            _ => MetadataType::Unknown,
            #[cfg(feature = "tests_deny_unknown_fields")]
            _ => return Err(serde::de::Error::unknown_variant(&metadata_type, &["movie", "episode", "photo", "show", "artist", "album", "season", "track", "collection", "clip", "playlist"])),
        })),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PlaylistMetadataType {
    Video,
    Audio,
    Photo,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(PlaylistMetadataType);

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CollectionMetadataSubtype {
    Show,
    Movie,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(CollectionMetadataSubtype);

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ClipMetadataSubtype {
    Trailer,
    DeletedScene,
    Interview,
    MusicVideo,
    BehindTheScenes,
    SceneOrSample,
    LiveMusicVideo,
    LyricMusicVideo,
    Concert,
    Featurette,
    Short,
    Other,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_fromstr_from_deserialize!(ClipMetadataSubtype);
