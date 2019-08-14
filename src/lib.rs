#![feature(async_await)]

//! # yt-api
//!
//! With the `yt-api` crate you can interact asynchronously with the youtube-api.
//!
//! ## Performing a search query
//!
//! To perform a search query, you can use the [`search`][search()] shortcut function.
//!
//! ```rust
//! # #![feature(async_await)]
//! # use std::env;
//! # use futures::future::{FutureExt, TryFutureExt};
//! # use yt_api::{
//! #     search::SearchList,
//! #     ApiKey,
//! # };
//! #
//! # fn main() {
//! let search_list = SearchList::new(ApiKey::new("your-youtube-api-key")).q("rust lang".to_string());
//!
//! let future = async move {
//!     let result = yt_api::search(&search_list).await.unwrap();
//! };
//!
//! tokio::run(future.unit_error().boxed().compat());
//! # }
//! ```

mod api;

use reqwest::r#async::Client;
use std::result::Result;

pub use api::*;

/// shortcut function to search for a video, channel or playlist
pub async fn search(
    query: &search::SearchList,
) -> Result<search::SearchListResponse, search::Error> {
    let client = Client::new();
    search::perform(client, query).await
}
