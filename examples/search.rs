#![feature(async_await)]
use std::env;

use futures::future::{FutureExt, TryFutureExt};
use yt_api::{
    search,
    search::SearchList,
    ApiKey,
};

// TODO: make this and the example in the README use tokio 0.2
/// prints the first answer of a search query
fn main() {
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_Key env-var not found"));

    // crate the SearchList struct for the query "rust lang"
    let search_list = SearchList::new(key).q("rust lang".to_string());

    let future = async move {
        // perform the search
        let result = search(&search_list).await.unwrap();
        // outputs the video_id of the first search result
        println!("https://youtube.com/watch?v={}", result.items[0].id.video_id.as_ref().unwrap());
    };

    // run the future
    tokio::run(future.unit_error().boxed().compat());
}
