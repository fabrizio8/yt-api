use std::env;

use yt_api::{
    search::{Error, ItemType, SearchList, VideoLocation},
    ApiKey,
};

/// prints the first answer of a search query
#[runtime::main]
async fn main() -> Result<(), Error> {
    // take api key from enviroment variable
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_KEY env-var not found"));

    // create the SearchList struct for the query "rust lang"
    let search_list = SearchList::new(key)
        .q("rust lang")
        .max_results(1)
        .item_type(ItemType::Video)
        .location(VideoLocation::new(40.73061, -73.93524))
        .location_radius("100km")
        .video_embeddable();

    // perform the search
    let result = search_list.await?;
    // outputs the video_id of the first search result
    println!(
        "Title: \"{}\"",
        result.items[0].snippet.title.as_ref().unwrap()
    );
    println!(
        "https://youtube.com/watch?v={}",
        result.items[0].id.video_id.as_ref().unwrap()
    );

    Ok(())
}
