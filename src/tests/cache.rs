use std::{collections::HashMap, io::Read, fs::File};

use crate::*;

#[tokio::test]
async fn cache_gets_recent() {
    let cache_path = "./src/tests/tmp/cache_tests_1.json";
    let prompt = "What's the deal with airplane food?";

    let mut client = OpenAIAccount::new(Opts {
        cache_filepath: cache_path.into(),
        ..Default::default()
    })
    .await
    .unwrap();

    client.cache.clear();

    print!("Assert that client in memory cache == an empty hash map: ");
    assert_eq!(client.cache.entries, HashMap::<String, Query>::new()); println!("Passed.");
    print!("Assert that client in memory cache length == 0: ");
    assert_eq!(client.cache.entries.len(), 0); println!("Passed.");

    let cache_file = std::fs::File::open(cache_path).unwrap();
    let can_deserialize_blank = serde_json::from_reader::<&File, HashMap<String, Query>>(&cache_file).is_ok();
    assert!(can_deserialize_blank); 


    let res = client.get_completion(prompt).await.expect("sucessful res");
    print!("Assert that response is not from cache: ");
    assert!(!res.from_cache); println!("Passed.");


    // Cache in memory and cache in file match 
    let cache_file = std::fs::File::open(cache_path).expect("To be able to open file");
    let cache: HashMap<String, Query> = serde_json::from_reader(&cache_file).expect("To be able to deserialize out of the cache file");
    
    print!("Assert that cache read from file matches the in memory cache: ");
    assert_eq!(cache, client.cache.entries); println!("Passed.");

    print!("Assert that the original completion we received has an id that matches the one we get when we get in memory cache at the intended key: ");
    assert_eq!(
        res.response.id.clone(), 
        client.cache.entries
            .get(&res.key())
            .expect("presence in cache")
            .clone()
            .response().id 
    ); println!("Passed.");
    
    let re_res = client.get_completion(prompt).await.expect("successful res 2");

    print!("Asser that the re-res of the same prompt gets labeled 'from cache': ");
    assert!(re_res.from_cache); println!("Passed."); // Did not re request

    let original = res.response.id;
    let redo = re_res.response.id;

    print!("Assert that the original completion we received has an id that matches the one we get when we get in memory cache at the intended key: ");
    assert_eq!(original, redo); // second should be same id as first

    
}