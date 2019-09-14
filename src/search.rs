use chrono::{DateTime, Utc};
use futures::{
    compat::{Future01CompatExt, Stream01CompatExt},
    TryStreamExt,
};
use log::debug;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize, Serializer};
use snafu::{ResultExt, Snafu};

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
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchList {
    key: ApiKey,
    part: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_content_owner: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_developer: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    for_mine: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_to_video_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_type: Option<ChannelType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<VideoLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of_content_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SearchOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relevance_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    safe_search: Option<SafeSearch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    item_type: Option<ItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_definition: Option<VideoDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_dimension: Option<VideoDimension>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    video_embeddable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_license: Option<VideoLicense>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    video_syndicated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_type: Option<VideoType>,
}

impl SearchList {
    const URL: &'static str = "https://www.googleapis.com/youtube/v3/search";

    /// create struct with an `yt_api::ApiKey`
    pub fn new(key: ApiKey) -> SearchList {
        SearchList {
            key,
            part: String::from("snippet"),
            for_content_owner: false,
            for_developer: false,
            for_mine: false,
            related_to_video_id: None,
            channel_id: None,
            channel_type: None,
            event_type: None,
            location: None,
            location_radius: None,
            max_results: None,
            on_behalf_of_content_owner: None,
            order: None,
            page_token: None,
            published_after: None,
            published_before: None,
            q: None,
            region_code: None,
            relevance_language: None,
            safe_search: None,
            topic_id: None,
            item_type: None,
            video_caption: None,
            video_category_id: None,
            video_definition: None,
            video_dimension: None,
            video_embeddable: false,
            video_license: None,
            video_syndicated: false,
            video_type: None,
        }
    }

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

    pub fn for_content_owner(mut self) -> Self {
        self.for_content_owner = true;
        self
    }

    pub fn for_developer(mut self) -> Self {
        self.for_developer = true;
        self
    }

    pub fn for_mine(mut self) -> Self {
        self.for_mine = true;
        self
    }

    pub fn related_to_video_id(mut self, related_to_video_id: impl Into<String>) -> Self {
        self.related_to_video_id = Some(related_to_video_id.into());
        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn channel_type(mut self, channel_type: impl Into<ChannelType>) -> Self {
        self.channel_type = Some(channel_type.into());
        self
    }

    pub fn event_type(mut self, event_type: impl Into<EventType>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    pub fn location(mut self, location: impl Into<VideoLocation>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn location_radius(mut self, location_radius: impl Into<String>) -> Self {
        self.location_radius = Some(location_radius.into());
        self
    }

    pub fn max_results(mut self, max_results: impl Into<u8>) -> Self {
        self.max_results = Some(max_results.into());
        self
    }

    pub fn on_behalf_of_content_owner(
        mut self,
        on_behalf_of_content_owner: impl Into<String>,
    ) -> Self {
        self.on_behalf_of_content_owner = Some(on_behalf_of_content_owner.into());
        self
    }

    pub fn order(mut self, order: impl Into<SearchOrder>) -> Self {
        self.order = Some(order.into());
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn published_after(mut self, published_after: impl Into<DateTime<Utc>>) -> Self {
        self.published_after = Some(published_after.into());
        self
    }

    pub fn published_before(mut self, published_before: impl Into<DateTime<Utc>>) -> Self {
        self.published_before = Some(published_before.into());
        self
    }

    pub fn q(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn region_code(mut self, region_code: impl Into<String>) -> Self {
        self.region_code = Some(region_code.into());
        self
    }

    pub fn relevance_language(mut self, relevance_language: impl Into<String>) -> Self {
        self.relevance_language = Some(relevance_language.into());
        self
    }

    pub fn safe_search(mut self, safe_search: impl Into<SafeSearch>) -> Self {
        self.safe_search = Some(safe_search.into());
        self
    }

    pub fn topic_id(mut self, topic_id: impl Into<String>) -> Self {
        self.topic_id = Some(topic_id.into());
        self
    }

    pub fn item_type(mut self, item_type: impl Into<ItemType>) -> Self {
        self.item_type = Some(item_type.into());
        self
    }

    pub fn video_caption(mut self, video_caption: impl Into<String>) -> Self {
        self.video_caption = Some(video_caption.into());
        self
    }

    pub fn video_category_id(mut self, video_category_id: impl Into<String>) -> Self {
        self.video_category_id = Some(video_category_id.into());
        self
    }

    pub fn video_definition(mut self, video_definition: impl Into<VideoDefinition>) -> Self {
        self.video_definition = Some(video_definition.into());
        self
    }

    pub fn video_dimension(mut self, video_dimension: impl Into<VideoDimension>) -> Self {
        self.video_dimension = Some(video_dimension.into());
        self
    }

    pub fn video_embeddable(mut self) -> Self {
        self.video_embeddable = true;
        self
    }

    pub fn video_license(mut self, video_license: impl Into<VideoLicense>) -> Self {
        self.video_license = Some(video_license.into());
        self
    }

    pub fn video_syndicated(mut self) -> Self {
        self.video_syndicated = true;
        self
    }

    pub fn video_type(mut self, video_type: impl Into<VideoType>) -> Self {
        self.video_type = Some(video_type.into());
        self
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
            latitude,
        }
    }
}

impl Serialize for VideoLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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
