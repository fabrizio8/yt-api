use chrono::{DateTime, Utc};
use futures::{
    compat::{Future01CompatExt, Stream01CompatExt},
    TryStreamExt,
};
use log::debug;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize, Serializer};
use snafu::{ResultExt, Snafu};
use typed_builder::TypedBuilder;

use super::ApiKey;

/// custom error type for the search endpoint
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to connect to the api: {}", source))]
    Connection { source: reqwest::Error },
    #[snafu(display("failed to deserialize: {} {}", string, source))]
    Deserialization {
        string: String,
        source: serde_json::Error,
    },
    #[snafu(display("failed to serialize: {}", source))]
    Serialization { source: serde_qs::Error },
}

/// request struct for the search endpoint
#[derive(Debug, TypedBuilder, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchList {
    key: ApiKey,
    #[builder(default_code = "String::from(\"snippet\")")]
    part: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_content_owner: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_developer: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_mine: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    related_to_video_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_type: Option<ChannelType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<EventType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<VideoLocation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    location_radius: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of_content_owner: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SearchOrder>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    published_after: Option<DateTime<Utc>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    published_before: Option<DateTime<Utc>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    region_code: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    relevance_language: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    safe_search: Option<SafeSearch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    item_type: Option<ItemType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_caption: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_category_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_definition: Option<VideoDefinition>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_dimension: Option<VideoDimension>,
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    video_embeddable: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_license: Option<VideoLicense>,
    #[builder(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    video_syndicated: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    video_type: Option<VideoType>,
}

impl SearchList {
    const URL: &'static str = "https://www.googleapis.com/youtube/v3/search";

    /// shorthand to perform the search with implicit `reqwest::async::Client` creation
    pub async fn perform(&self) -> Result<SearchListResponse, Error> {
        let client = Client::new();
        self.perform_with(client).await
    }

    /// searches for a video, channel or playlist
    pub async fn perform_with(&self, client: Client) -> Result<SearchListResponse, Error> {
        let url = format!(
            "{}?{}",
            Self::URL,
            serde_qs::to_string(&self).context(Serialization)?
        );
        debug!("getting {}", url);
        let response = client.get(&url).send().compat().await.context(Connection)?;

        let chunks = response
            .into_body()
            .compat()
            .try_concat()
            .await
            .context(Connection)?;
        let response = String::from_utf8_lossy(&chunks);

        serde_json::from_str(&response).context(Deserialization { string: response })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelType {
    Any,
    Show,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Completed,
    Live,
    Upcoming,
}

#[derive(Debug, Clone)]
pub struct VideoLocation {
    longitude: f32,
    latitude: f32,
}

impl VideoLocation {
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self {
            longitude,
            latitude
        }
    }
}

impl Serialize for VideoLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        serializer.serialize_str(&format!("{},{}", self.longitude, self.latitude))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchOrder {
    Date,
    Rating,
    Relevance,
    Title,
    VideoCount,
    ViewCount,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SafeSearch {
    Moderate,
    Strict,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Channel,
    Playlist,
    Video,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoCaption {
    ClosedCaption,
    None,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoDefinition {
    High,
    Standard,
}

#[derive(Debug, Clone, Serialize)]
pub enum VideoDimension {
    #[serde(rename = "3d")]
    Three,
    #[serde(rename = "2d")]
    Two,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoDuration {
    Long,
    Medium,
    Short,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoLicense {
    CreativeCommon,
    Youtube,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoType {
    Episode,
    Movie,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchListResponse {
    pub kind: String,
    pub etag: String,
    pub prev_page_token: Option<String>,
    pub region_code: String,
    pub page_info: PageInfo,
    pub items: Vec<SearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: i64,
    pub results_per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub kind: String,
    pub etag: String,
    pub id: SearchResultId,
    pub snippet: SearchResultSnippet,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultId {
    pub kind: String,
    pub video_id: Option<String>,
    pub channel_id: Option<String>,
    pub playlist_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultSnippet {
    pub published_at: Option<DateTime<Utc>>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnails: Option<SearchResultThumbnails>,
    pub channel_title: Option<String>,
    pub live_broadcast_content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResultThumbnails {
    default: Option<SearchResultThumbnail>,
    medium: Option<SearchResultThumbnail>,
    high: Option<SearchResultThumbnail>,
    standard: Option<SearchResultThumbnail>,
    maxres: Option<SearchResultThumbnail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResultThumbnail {
    pub url: String,
    pub width: Option<u64>,
    pub height: Option<u64>,
}
