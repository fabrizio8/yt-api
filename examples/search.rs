use std::env;

use futures::future::{FutureExt, TryFutureExt};
use yt_api::{
    search::{ItemType, SearchList},
    ApiKey,
};

/// prints the first answer of a search query
fn main() {
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_KEY env-var not found"));

    // create the SearchList struct for the query "rust lang"
    let search_list = SearchList::new(key)
        .q("rust lang")
        .item_type(ItemType::Video);

    let future = async move {
        // perform the search
        let result = search_list.perform().await.unwrap();
        // outputs the title of the first search result
        println!(
            "Title: \"{}\"",
            result.items[0].snippet.title.as_ref().unwrap()
        );
        // outputs the video id of the first search result
        println!(
            "https://youtube.com/watch?v={}",
            result.items[0].id.video_id.as_ref().unwrap()
        );
    };

    // run the future
    tokio::run(future.unit_error().boxed().compat());
}
