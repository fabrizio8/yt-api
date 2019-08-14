use futures::{
    compat::{Future01CompatExt, Stream01CompatExt},
    TryStreamExt,
};
use log::debug;
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use chrono::DateTime;

use super::ApiKey;

const URL: &str = "https://www.googleapis.com/youtube/v3/search";

/// searches for a video, channel or playlist
pub async fn perform(client: Client, query: &SearchList) -> Result<SearchListResponse, Error> {
    let url = format!(
        "{}?{}",
        URL,
        serde_qs::to_string(&query).context(Serialization)?
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
    #[snafu(display("failed to build: missing fields"))]
    BuilderMissingField,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchList {
    key: ApiKey,
    part: String,
    q: Option<String>,
}

impl SearchList {
    pub fn new(key: ApiKey) -> SearchList {
        SearchList {
            key: key,
            part: String::from("snippet"),
            q: None,
        }
    }

    pub fn q(mut self, q: String) -> SearchList {
        self.q = Some(q);
        self
    }
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
    pub published_at: Option<DateTime>,
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
