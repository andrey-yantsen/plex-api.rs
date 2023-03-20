//! Support for transcoding media files into lower quality versions.
//!
//! Transcoding comes in two forms:
//! * Streaming allows for real-time playback of the media using streaming
//!   protocols such as [HTTP Live Streaming](https://en.wikipedia.org/wiki/HTTP_Live_Streaming)
//!   and [Dynamic Adaptive Streaming over HTTP](https://en.wikipedia.org/wiki/Dynamic_Adaptive_Streaming_over_HTTP).
//! * Offline transcoding (the mobile downloads feature) requests that the
//!   server converts the file in the background allowing it to be downloaded
//!   later.
//!
//! This feature should be considered quite experimental, lots of the API calls
//! are derived from inspection and guesswork.
use std::{collections::HashMap, fmt::Display};

use futures::AsyncWrite;
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;
use uuid::Uuid;

use crate::{
    error,
    media_container::{
        server::{
            library::{
                AudioStream, Decision, Media as MediaMetadata, Metadata, Protocol, Stream,
                VideoStream,
            },
            Feature,
        },
        MediaContainer, MediaContainerWrapper,
    },
    server::library::{MediaItemWithTranscoding, Part},
    url::{SERVER_TRANSCODE_ART, SERVER_TRANSCODE_DECISION, SERVER_TRANSCODE_DOWNLOAD},
    AudioCodec, ContainerFormat, HttpClient, Result, VideoCodec,
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Context {
    Streaming,
    Static,
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    Unknown,
}

derive_display_from_serialize!(Context);

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct TranscodeDecisionMediaContainer {
    general_decision_code: Option<u32>,
    general_decision_text: Option<String>,

    direct_play_decision_code: Option<u32>,
    direct_play_decision_text: Option<String>,

    transcode_decision_code: Option<u32>,
    transcode_decision_text: Option<String>,

    allow_sync: String,
    #[serde(rename = "librarySectionID")]
    library_section_id: Option<String>,
    library_section_title: Option<String>,
    #[serde(rename = "librarySectionUUID")]
    library_section_uuid: Option<String>,
    media_tag_prefix: Option<String>,
    media_tag_version: Option<String>,
    resource_session: Option<String>,

    #[serde(flatten)]
    media_container: MediaContainer,

    #[serde(default, rename = "Metadata")]
    metadata: Vec<Metadata>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct TranscodeSessionStats {
    pub key: String,
    pub throttled: bool,
    pub complete: bool,
    // Percentage complete.
    pub progress: f32,
    pub size: i64,
    pub speed: Option<f32>,
    pub error: bool,
    pub duration: Option<u32>,
    // Appears to be the number of seconds that the server thinks remain.
    pub remaining: Option<u32>,
    pub context: Context,
    pub source_video_codec: Option<VideoCodec>,
    pub source_audio_codec: Option<AudioCodec>,
    pub video_decision: Option<Decision>,
    pub audio_decision: Option<Decision>,
    pub subtitle_decision: Option<Decision>,
    pub protocol: Protocol,
    pub container: ContainerFormat,
    pub video_codec: Option<VideoCodec>,
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: u8,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub transcode_hw_requested: bool,
    pub transcode_hw_decoding: Option<String>,
    pub transcode_hw_encoding: Option<String>,
    pub transcode_hw_decoding_title: Option<String>,
    pub transcode_hw_full_pipeline: Option<bool>,
    pub transcode_hw_encoding_title: Option<String>,
    #[serde(default)]
    pub offline_transcode: bool,
    pub time_stamp: Option<f32>,
    pub min_offset_available: Option<f32>,
    pub max_offset_available: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TranscodeSessionsMediaContainer {
    #[serde(default, rename = "TranscodeSession")]
    pub transcode_sessions: Vec<TranscodeSessionStats>,
}

struct Query {
    params: HashMap<String, String>,
}

impl Query {
    fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    fn param<N: Into<String>, V: Into<String>>(mut self, name: N, value: V) -> Self {
        self.params.insert(name.into(), value.into());
        self
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        serde_urlencoded::to_string(&self.params).unwrap()
    }
}

struct ProfileSetting {
    setting: String,
    params: Vec<String>,
}

impl ProfileSetting {
    fn new(setting: &str) -> Self {
        Self {
            setting: setting.to_owned(),
            params: Vec::new(),
        }
    }

    fn param<N: Display, V: Display>(mut self, name: N, value: V) -> Self {
        self.params.push(format!("{name}={value}"));
        self
    }
}

impl ToString for ProfileSetting {
    fn to_string(&self) -> String {
        format!("{}({})", self.setting, self.params.join("&"))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VideoSetting {
    /// Video width.
    Width,
    /// Video height.
    Height,
    /// Colour bit depth.
    BitDepth,
    /// h264 level.
    Level,
    /// Supported h264 profile.
    Profile,
    /// Framerate.
    FrameRate,
}

impl ToString for VideoSetting {
    fn to_string(&self) -> String {
        match self {
            VideoSetting::Width => "video.width".to_string(),
            VideoSetting::Height => "video.height".to_string(),
            VideoSetting::BitDepth => "video.bitDepth".to_string(),
            VideoSetting::Level => "video.level".to_string(),
            VideoSetting::Profile => "video.profile".to_string(),
            VideoSetting::FrameRate => "video.frameRate".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AudioSetting {
    /// Audio channels.
    Channels,
    /// Sample rate.
    SamplingRate,
    /// Sample bit depth.
    BitDepth,
}

impl ToString for AudioSetting {
    fn to_string(&self) -> String {
        match self {
            AudioSetting::Channels => "audio.channels".to_string(),
            AudioSetting::SamplingRate => "audio.samplingRate".to_string(),
            AudioSetting::BitDepth => "audio.bitDepth".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Max(String),
    Min(String),
    Match(Vec<String>),
    NotMatch(String),
}

/// Limitations add a constraint to the supported media format.
///
/// They generally set the maximum or minimum value of a setting or constrain
/// the setting to a specific list of values. So for example you can set the
/// maximum video width or the maximum number of audio channels. Limitations are
/// either set on a per-codec basis or apply to all codecs.
#[derive(Debug, Clone)]
pub struct Limitation<C, S> {
    pub codec: Option<C>,
    pub setting: S,
    pub constraint: Constraint,
}

impl<C: ToString, S: ToString> Limitation<C, S> {
    fn build(&self, scope: &str) -> ProfileSetting {
        let scope_name = if let Some(codec) = &self.codec {
            codec.to_string()
        } else {
            "*".to_string()
        };
        let name = self.setting.to_string();

        let setting = ProfileSetting::new("add-limitation")
            .param("scope", scope)
            .param("scopeName", scope_name)
            .param("name", name);

        match &self.constraint {
            Constraint::Max(v) => setting.param("type", "upperBound").param("value", v),
            Constraint::Min(v) => setting.param("type", "lowerBound").param("value", v),
            Constraint::Match(l) => setting.param("type", "match").param(
                "list",
                l.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("|"),
            ),
            Constraint::NotMatch(v) => setting.param("type", "notMatch").param("value", v),
        }
    }
}

impl<C, S> From<(S, Constraint)> for Limitation<C, S> {
    fn from((setting, constraint): (S, Constraint)) -> Self {
        Self {
            codec: None,
            setting,
            constraint,
        }
    }
}

impl<C, S> From<(C, S, Constraint)> for Limitation<C, S> {
    fn from((codec, setting, constraint): (C, S, Constraint)) -> Self {
        Self {
            codec: Some(codec),
            setting,
            constraint,
        }
    }
}

impl<C, S> From<(Option<C>, S, Constraint)> for Limitation<C, S> {
    fn from((codec, setting, constraint): (Option<C>, S, Constraint)) -> Self {
        Self {
            codec,
            setting,
            constraint,
        }
    }
}

pub trait TranscodeOptions {
    fn transcode_parameters(
        &self,
        context: Context,
        protocol: Protocol,
        container: Option<ContainerFormat>,
    ) -> String;
}

/// Defines the media formats suitable for transcoding video. The server uses
/// these settings to choose a format to transcode to.
///
/// The server is not very clever at choosing codecs that work for a given
/// container format. It is safest to only list codecs and containers that work
/// together.
///
/// Note that the server maintains default transcode profiles for many devices
/// which will alter the supported transcode targets. By default for instance if
/// the server thinks you are an Android client it will only offer stereo audio
/// in videos. You can see these profiles in `Resources/Profiles` of the media
/// server install directory. Individual settings in the profile can be
/// overridden via the API however if you want to be sure of a clean slate use
/// a [generic client](crate::HttpClientBuilder::generic).
#[derive(Debug, Clone)]
pub struct VideoTranscodeOptions {
    /// Maximum bitrate in kbps.
    pub bitrate: u32,
    /// Maximum video width.
    pub width: u32,
    /// Maximum video height.
    pub height: u32,
    /// Audio gain from 0 to 100.
    pub audio_boost: Option<u8>,
    /// Whether to burn the subtitles into the video.
    pub burn_subtitles: bool,
    /// Supported media container formats. Ignored for streaming transcodes.
    pub containers: Vec<ContainerFormat>,
    /// Supported video codecs.
    pub video_codecs: Vec<VideoCodec>,
    /// Limitations to constraint video transcoding options.
    pub video_limitations: Vec<Limitation<VideoCodec, VideoSetting>>,
    /// Supported audio codecs.
    pub audio_codecs: Vec<AudioCodec>,
    /// Limitations to constraint audio transcoding options.
    pub audio_limitations: Vec<Limitation<AudioCodec, AudioSetting>>,
}

impl Default for VideoTranscodeOptions {
    fn default() -> Self {
        Self {
            bitrate: 2000,
            width: 1280,
            height: 720,
            audio_boost: None,
            burn_subtitles: true,
            containers: vec![ContainerFormat::Mp4, ContainerFormat::Mkv],
            video_codecs: vec![VideoCodec::H264],
            video_limitations: Default::default(),
            audio_codecs: vec![AudioCodec::Aac, AudioCodec::Mp3],
            audio_limitations: Default::default(),
        }
    }
}

impl TranscodeOptions for VideoTranscodeOptions {
    fn transcode_parameters(
        &self,
        context: Context,
        protocol: Protocol,
        container: Option<ContainerFormat>,
    ) -> String {
        let mut query = Query::new()
            .param("maxVideoBitrate", self.bitrate.to_string())
            .param("videoBitrate", self.bitrate.to_string())
            .param("videoResolution", format!("{}x{}", self.width, self.height));

        if self.burn_subtitles {
            query = query
                .param("subtitles", "burn")
                .param("subtitleSize", "100");
        }

        if let Some(boost) = self.audio_boost {
            query = query.param("audioBoost", boost.to_string());
        }

        let video_codecs = self
            .video_codecs
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let audio_codecs = self
            .audio_codecs
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let containers = if let Some(container) = container {
            vec![container.to_string()]
        } else {
            self.containers.iter().map(ToString::to_string).collect()
        };

        let mut profile = Vec::new();

        for container in containers {
            profile.push(
                ProfileSetting::new("add-transcode-target")
                    .param("type", "videoProfile")
                    .param("context", context.to_string())
                    .param("protocol", protocol.to_string())
                    .param("container", &container)
                    .param("videoCodec", &video_codecs)
                    .param("audioCodec", &audio_codecs)
                    .to_string(),
            );

            // Allow potentially direct playing for offline transcodes.
            if context == Context::Static {
                profile.push(
                    ProfileSetting::new("add-direct-play-profile")
                        .param("type", "videoProfile")
                        .param("container", container)
                        .param("videoCodec", &video_codecs)
                        .param("audioCodec", &audio_codecs)
                        .to_string(),
                );
            }
        }

        profile.extend(self.video_codecs.iter().map(|codec| {
            ProfileSetting::new("append-transcode-target-codec")
                .param("type", "videoProfile")
                .param("context", context.to_string())
                .param("protocol", protocol.to_string())
                .param("videoCodec", codec.to_string())
                .to_string()
        }));

        profile.extend(self.audio_codecs.iter().map(|codec| {
            ProfileSetting::new("add-transcode-target-audio-codec")
                .param("type", "videoProfile")
                .param("context", context.to_string())
                .param("protocol", protocol.to_string())
                .param("audioCodec", codec.to_string())
                .to_string()
        }));

        profile.extend(
            self.video_limitations
                .iter()
                .map(|l| l.build("videoCodec").to_string()),
        );
        profile.extend(
            self.audio_limitations
                .iter()
                .map(|l| l.build("videoAudioCodec").to_string()),
        );

        query
            .param("X-Plex-Client-Profile-Extra", profile.join("+"))
            .to_string()
    }
}

/// Defines the media formats suitable for transcoding music. The server uses
/// these settings to choose a format to transcode to.
///
/// The server is not very clever at choosing codecs that work for a given
/// container format. It is safest to only list codecs and containers that work
/// together.
///
/// Note that the server maintains default transcode profiles for many devices
/// which will alter the supported transcode targets. By default for instance if
/// the server thinks you are an Android client it will only offer stereo audio
/// in videos. You can see these profiles in `Resources/Profiles` of the media
/// server install directory. Individual settings in the profile can be
/// overridden via the API however if you want to be sure of a clean slate use
/// a [generic client](crate::HttpClientBuilder::generic).
#[derive(Debug, Clone)]
pub struct MusicTranscodeOptions {
    /// Maximum bitrate in kbps.
    pub bitrate: u32,
    /// Supported media container formats. Ignored for streaming transcodes.
    pub containers: Vec<ContainerFormat>,
    /// Supported audio codecs.
    pub codecs: Vec<AudioCodec>,
    /// Limitations to constraint audio transcoding options.
    pub limitations: Vec<Limitation<AudioCodec, AudioSetting>>,
}

impl Default for MusicTranscodeOptions {
    fn default() -> Self {
        Self {
            bitrate: 192,
            containers: vec![ContainerFormat::Mp3],
            codecs: vec![AudioCodec::Mp3],
            limitations: Default::default(),
        }
    }
}

impl TranscodeOptions for MusicTranscodeOptions {
    fn transcode_parameters(
        &self,
        context: Context,
        protocol: Protocol,
        container: Option<ContainerFormat>,
    ) -> String {
        let query = Query::new().param("musicBitrate", self.bitrate.to_string());

        let audio_codecs = self
            .codecs
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let containers = if let Some(container) = container {
            vec![container.to_string()]
        } else {
            self.containers.iter().map(ToString::to_string).collect()
        };

        let mut profile = Vec::new();

        for container in containers {
            profile.push(
                ProfileSetting::new("add-transcode-target")
                    .param("type", "musicProfile")
                    .param("context", context.to_string())
                    .param("protocol", protocol.to_string())
                    .param("container", &container)
                    .param("audioCodec", &audio_codecs)
                    .to_string(),
            );

            // Allow potentially direct playing for offline transcodes.
            if context == Context::Static {
                profile.push(
                    ProfileSetting::new("add-direct-play-profile")
                        .param("type", "musicProfile")
                        .param("container", container)
                        .param("audioCodec", &audio_codecs)
                        .to_string(),
                );
            }
        }

        profile.extend(
            self.limitations
                .iter()
                .map(|l| l.build("audioCodec").to_string()),
        );

        query
            .param("X-Plex-Client-Profile-Extra", profile.join("+"))
            .to_string()
    }
}

/// Generates a unique session id. This appears to just be any random string.
fn session_id() -> String {
    Uuid::new_v4().as_simple().to_string()
}

fn bs(val: bool) -> String {
    if val {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

fn get_transcode_params<M: MediaItemWithTranscoding>(
    id: &str,
    context: Context,
    protocol: Protocol,
    item_metadata: &Metadata,
    part: &Part<M>,
    options: M::Options,
) -> Result<String> {
    let container = match (context, protocol) {
        (Context::Static, _) => None,
        (_, Protocol::Dash) => Some(ContainerFormat::Mp4),
        (_, Protocol::Hls) => Some(ContainerFormat::MpegTs),
        _ => return Err(error::Error::InvalidTranscodeSettings),
    };

    let mut query = Query::new()
        .param("session", id)
        .param("path", item_metadata.key.clone())
        .param("mediaIndex", part.media_index.to_string())
        .param("partIndex", part.part_index.to_string())
        // Setting this to true tells the server that we're willing to directly
        // play the item if needed. That probably makes sense for downloads but
        // not streaming (where we need the DASH/HLS protocol).
        .param("directPlay", bs(context == Context::Static))
        // Allows using the original video stream if possible.
        .param("directStream", bs(true))
        // Allows using the original audio stream if possible.
        .param("directStreamAudio", bs(true))
        .param("protocol", protocol.to_string())
        .param("context", context.to_string())
        .param("location", "lan")
        .param("fastSeek", bs(true));

    if context == Context::Static {
        query = query.param("offlineTranscode", bs(true));
    }

    let query = query.to_string();

    let params = options.transcode_parameters(context, protocol, container);

    Ok(format!("{query}&{params}"))
}

async fn transcode_decision<'a, M: MediaItemWithTranscoding>(
    part: &Part<'a, M>,
    params: &str,
) -> Result<MediaMetadata> {
    let path = format!("{SERVER_TRANSCODE_DECISION}?{params}");

    let mut response = part
        .client
        .get(path)
        .header("Accept", "application/json")
        .send()
        .await?;

    let text = match response.status() {
        StatusCode::OK => response.text().await?,
        _ => return Err(crate::Error::from_response(response).await),
    };

    let wrapper: MediaContainerWrapper<TranscodeDecisionMediaContainer> =
        serde_json::from_str(&text)?;

    if wrapper.media_container.general_decision_code == Some(2011)
        && wrapper.media_container.general_decision_text
            == Some("Downloads not allowed".to_string())
    {
        return Err(error::Error::SubscriptionFeatureNotAvailable(
            Feature::SyncV3,
        ));
    }

    if wrapper.media_container.direct_play_decision_code == Some(1000) {
        return Err(error::Error::TranscodeRefused);
    }

    wrapper
        .media_container
        .metadata
        .into_iter()
        .next()
        .and_then(|m| m.media)
        .and_then(|m| m.into_iter().find(|m| m.selected == Some(true)))
        .ok_or_else(|| {
            if let Some(text) = wrapper.media_container.transcode_decision_text {
                error::Error::TranscodeError(text)
            } else {
                error::Error::UnexpectedApiResponse {
                    status_code: response.status().as_u16(),
                    content: text,
                }
            }
        })
}

pub(super) async fn create_transcode_session<'a, M: MediaItemWithTranscoding>(
    item_metadata: &'a Metadata,
    part: &Part<'a, M>,
    context: Context,
    target_protocol: Protocol,
    options: M::Options,
) -> Result<TranscodeSession> {
    let id = session_id();

    let params = get_transcode_params(&id, context, target_protocol, item_metadata, part, options)?;

    let media_data = transcode_decision(part, &params).await?;

    if target_protocol != media_data.protocol.unwrap_or(Protocol::Http) {
        return Err(error::Error::TranscodeError(
            "Server returned an invalid protocol.".to_string(),
        ));
    }

    TranscodeSession::from_metadata(
        id,
        part.client.clone(),
        media_data,
        context == Context::Static,
        params,
    )
}

pub enum TranscodeStatus {
    Complete,
    Error,
    Transcoding {
        // The server's estimate of how many seconds are left until complete.
        remaining: Option<u32>,
        // Percent complete (0-100).
        progress: f32,
    },
}

pub struct TranscodeSession {
    id: String,
    client: HttpClient,
    offline: bool,
    protocol: Protocol,
    container: ContainerFormat,
    video_transcode: Option<(Decision, VideoCodec)>,
    audio_transcode: Option<(Decision, AudioCodec)>,
    params: String,
}

impl TranscodeSession {
    pub(crate) fn from_stats(client: HttpClient, stats: TranscodeSessionStats) -> Self {
        Self {
            client,
            // Once the transcode session is started we only need the session ID
            // to download.
            params: format!("session={}", stats.key),
            offline: stats.offline_transcode,
            container: stats.container,
            protocol: stats.protocol,
            video_transcode: stats.video_decision.zip(stats.video_codec),
            audio_transcode: stats.audio_decision.zip(stats.audio_codec),
            id: stats.key,
        }
    }

    fn from_metadata(
        id: String,
        client: HttpClient,
        media_data: MediaMetadata,
        offline: bool,
        params: String,
    ) -> Result<Self> {
        let part_data = media_data
            .parts
            .iter()
            .find(|p| p.selected == Some(true))
            .ok_or_else(|| {
                error::Error::TranscodeError("Server returned unexpected response".to_string())
            })?;

        let streams = part_data.streams.as_ref().ok_or_else(|| {
            error::Error::TranscodeError("Server returned unexpected response".to_string())
        })?;

        let video_streams = streams
            .iter()
            .filter_map(|s| match s {
                Stream::Video(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<&VideoStream>>();

        let video_transcode = video_streams
            .iter()
            .find(|s| s.selected == Some(true))
            .or_else(|| video_streams.get(0))
            .map(|s| (s.decision.unwrap(), s.codec));

        let audio_streams = streams
            .iter()
            .filter_map(|s| match s {
                Stream::Audio(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<&AudioStream>>();

        let audio_transcode = audio_streams
            .iter()
            .find(|s| s.selected == Some(true))
            .or_else(|| audio_streams.get(0))
            .map(|s| (s.decision.unwrap(), s.codec));

        Ok(Self {
            id,
            client,
            offline,
            params,
            container: media_data.container.unwrap(),
            protocol: media_data.protocol.unwrap_or(Protocol::Http),
            video_transcode,
            audio_transcode,
        })
    }

    /// The session ID allows for re-retrieving this session at a later date.
    pub fn session_id(&self) -> &str {
        &self.id
    }

    pub fn is_offline(&self) -> bool {
        self.offline
    }

    /// The selected protocol.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// The selected container.
    pub fn container(&self) -> ContainerFormat {
        self.container
    }

    // The target video codec and the transcode decision.
    pub fn video_transcode(&self) -> Option<(Decision, VideoCodec)> {
        self.video_transcode
    }

    // The target audio codec and the transcode decision.
    pub fn audio_transcode(&self) -> Option<(Decision, AudioCodec)> {
        self.audio_transcode
    }

    /// Downloads the transcoded data to the provided writer.
    ///
    /// For streaming transcodes (MPEG-DASH or HLS) this will return the
    /// playlist data. This crate doesn't contain any support for processing
    /// these streaming formats and figuring out how to use them is currently
    /// left as an exercise for the caller.
    ///
    /// For offline transcodes it is possible to start downloading before the
    /// transcode is complete. In this case any data already transcoded is
    /// downloaded and then the connection will remain open and more data will
    /// be delivered to the writer as it becomes available. This can mean
    /// that the HTTP connection is idle for long periods of time waiting for
    /// more data to be transcoded and so the normal timeouts are disabled for
    /// offline transcode downloads.
    ///
    /// Unfortunately there does not appear to be any way to restart downloads
    /// from a specific point in the file. So if the download fails for
    /// any reason you have to start downloading all over again. It may make
    /// more sense to wait until the transcode is complete or nearly complete
    /// before attempting download.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn download<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Strictly speaking it doesn't appear that the requested extension
        // matters but we'll attempt to match other clients anyway.
        let ext = match (self.protocol, self.container) {
            (Protocol::Dash, _) => "mpd".to_string(),
            (Protocol::Hls, _) => "m3u8".to_string(),
            (_, container) => container.to_string(),
        };

        let path = format!("{SERVER_TRANSCODE_DOWNLOAD}/start.{}?{}", ext, self.params);

        let mut builder = self.client.get(path);
        if self.offline {
            builder = builder.timeout(None)
        }
        let mut response = builder.send().await?;

        match response.status() {
            StatusCode::OK => {
                response.copy_to(writer).await?;
                Ok(())
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn status(&self) -> Result<TranscodeStatus> {
        let stats = self.stats().await?;

        if stats.error {
            Ok(TranscodeStatus::Error)
        } else if stats.complete {
            Ok(TranscodeStatus::Complete)
        } else {
            Ok(TranscodeStatus::Transcoding {
                remaining: stats.remaining,
                progress: stats.progress,
            })
        }
    }

    /// Retrieves the current transcode stats.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn stats(&self) -> Result<TranscodeSessionStats> {
        let wrapper: MediaContainerWrapper<TranscodeSessionsMediaContainer> = self
            .client
            .get(format!("/transcode/sessions/{}", self.id))
            .json()
            .await?;
        wrapper
            .media_container
            .transcode_sessions
            .get(0)
            .cloned()
            .ok_or(crate::Error::ItemNotFound)
    }

    /// Cancels the transcode and removes any transcoded data from the server.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn cancel(self) -> Result<()> {
        let mut response = self
            .client
            .get(format!(
                "/video/:/transcode/universal/stop?session={}",
                self.id
            ))
            .send()
            .await?;

        match response.status() {
            // Sometimes the server will respond not found but still cancel the
            // session.
            StatusCode::OK | StatusCode::NOT_FOUND => Ok(response.consume().await?),
            _ => Err(crate::Error::from_response(response).await),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArtTranscodeOptions {
    /// If true and the source image is smaller than that requested it will be
    /// upscaled.
    pub upscale: bool,
    /// Sets whether the requested size is the minimum size desired or the
    /// maximum.
    pub min_size: bool,
}

impl Default for ArtTranscodeOptions {
    fn default() -> Self {
        Self {
            upscale: true,
            min_size: true,
        }
    }
}

pub(crate) async fn transcode_artwork<W>(
    client: &HttpClient,
    art: &str,
    width: u32,
    height: u32,
    options: ArtTranscodeOptions,
    writer: W,
) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let query = Query::new()
        .param("url", art)
        .param("upscale", bs(options.upscale))
        .param("minSize", bs(options.min_size))
        .param("width", width.to_string())
        .param("height", height.to_string());

    let mut response = client
        .get(format!("{SERVER_TRANSCODE_ART}?{}", query.to_string()))
        .send()
        .await?;

    match response.status() {
        // Sometimes the server will respond not found but still cancel the
        // session.
        StatusCode::OK => {
            response.copy_to(writer).await?;
            Ok(())
        }
        _ => Err(crate::Error::from_response(response).await),
    }
}
