# yt-api

[![Crates.io](https://img.shields.io/crates/v/yt-api.svg)](https://crates.io/crates/yt-api)
[![Documentation](https://docs.rs/yt-api/badge.svg)](https://docs.rs/yt-api)
[![dependency status](https://deps.rs/repo/gitlab/nycex/yt-api/status.svg)](https://deps.rs/repo/gitlab/nycex/yt-api)
[![pipeline status](https://gitlab.com/nycex/yt-api/badges/master/build.svg)](https://gitlab.com/nycex/yt-api/pipelines)

## about
With yt-api you can interact asynchronously with the youtube-api.
Currently it implements the following endpoints:
 * search
 
## example
A basic search request with yt-api:

``` rust
/// prints the first answer of a search query
#[runtime::main]
async fn main() -> Result<(), Error> {
    // take api key from enviroment variable
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_KEY env-var not found"));

    // create the SearchList struct for the query "rust lang"
    let search_list = SearchList::new(key)
        .q("rust lang")
        .item_type(ItemType::Video);

    // perform the search
    let result = search_list.perform().await?;
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

    Ok(())
}
```

More examples can be found [here](examples). 

## supported rust versions

the minimum rust version for yt-api is 1.39

## license

This project is licensed under the [MIT license](LICENSE).

## contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in yt-api by you, shall be licensed as MIT, without any additional
terms or conditions.

