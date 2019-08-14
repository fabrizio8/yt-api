#![feature(async_await)]
mod api;

use reqwest::r#async::Client;
use std::result::Result;

pub use api::*;

/// shorthand to search for a video, channel or playlist
pub async fn search(
    query: &search::SearchList,
) -> Result<search::SearchListResponse, search::Error> {
    let client = Client::new();
    search::perform(client, query).await
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
