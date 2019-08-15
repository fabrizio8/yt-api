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
fn main() {
    let key = ApiKey::new(&env::var("YT_API_KEY").expect("YT_API_Key env-var not found"));

    // create the SearchList struct for the query "rust lang"
    let search_list = SearchList::builder()
        .key(key)
        .q("rust lang".to_string())
        .item_type(ItemType::Video)
        .build();

    let future = async move {
        // perform the search
        let result = search_list.perform().await.unwrap();
        // outputs the video_id of the first search result
        println!(
            "https://youtube.com/watch?v={}",
            result.items[0].id.video_id.as_ref().unwrap()
        );
    };

    // run the future
    tokio::run(future.unit_error().boxed().compat());
}
```

More examples can be found [here](examples). 

## supported rust versions

yt-api is currently only able to run on nightly, but as async/await will
probably be stabilized in 1.38, this will be the required rust version.

## license

This project is licensed under the [MIT license](LICENSE).

## contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in yt-api by you, shall be licensed as MIT, without any additional
terms or conditions.

