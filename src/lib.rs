//! # yt-api
//!
//! With the `yt-api` crate you can interact asynchronously with the youtube-api.
//!
//! ## Performing a search query
//!
//! To perform a search query, you can create a [`SearchList`][search_list] query.
//!
//! ```rust
//! # use yt_api::{
//! #     search::SearchList,
//! #     ApiKey,
//! # };
//! #
//! # #[runtime::main]
//! # async fn main() {
//! let search_list = SearchList::new(ApiKey::new("your-youtube-api-key")).q("rust lang");
//!
//! let result = search_list.await;
//! # }
//! ```
//!
//! [search_list]: ./search/struct.SearchList.html
//! [search_perform]: ./search/struct.SearchList.html#method.perform

pub mod search;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
}
