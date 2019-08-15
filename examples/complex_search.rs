#![feature(async_await)]
use std::env;

use futures::future::{FutureExt, TryFutureExt};
use yt_api::{
    search::{SearchItemType, SearchList, VideoLocation},
    ApiKey,
};

/// prints the first answer of a search query
fn main() {
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_Key env-var not found"));

    // create the SearchList struct for the query "rust lang"
    let search_list = SearchList::builder()
        .key(key)
        .q("rust lang".to_string())
        .max_results(1)
        .search_type(SearchItemType::Video)
        .location(VideoLocation::new(40.73061, -73.93524))
        .location_radius("100km".to_string())
        .video_embeddable(true)
        .build();

    let future = async move {
        // perform the search
        let result = search_list.perform().await.unwrap();
        // outputs the video_id of the first search result
        println!(
            "Title: \"{}\"",
            result.items[0].snippet.title.as_ref().unwrap()
        );
        println!(
            "https://youtube.com/watch?v={}",
            result.items[0].id.video_id.as_ref().unwrap()
        );
    };

    // run the future
    tokio::run(future.unit_error().boxed().compat());
}
