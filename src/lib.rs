//! # yt-api
//!
//! With the `yt-api` crate you can interact asynchronously with the youtube-api.
//!
//! ## Performing a search query
//!
//! To perform a search query, you can use the [`perform`][search_perform] function on the [`SearchList`][search_list] query.
//!
//! ```rust
//! # use std::env;
//! # use futures::future::{FutureExt, TryFutureExt};
//! # use yt_api::{
//! #     search::SearchList,
//! #     ApiKey,
//! # };
//! #
//! # fn main() {
//! let search_list = SearchList::new(ApiKey::new("your-youtube-api-key")).q("rust lang");
//!
//! let future = async move {
//!     let result = search_list.perform().await.unwrap();
//! };
//!
//! tokio::run(future.unit_error().boxed().compat());
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
    pub fn new(key: impl Into<String>) -> ApiKey {
        ApiKey(key.into())
    }
}
