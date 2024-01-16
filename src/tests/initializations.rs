use std::io::Read;

use crate::*;

#[tokio::test]
async fn default() {
    let client = OpenAIAccount::new(Opts::default()).await.unwrap();
}

#[tokio::test]
async fn altered_cache_bill() {
    let client = OpenAIAccount::new(Opts { cache_filepath: "./src/tests/tmp/cache.json".into(),  ..Default::default()}).await.unwrap();
    assert!(std::fs::File::open("./src/tests/tmp/cache.json").is_ok());
}

#[tokio::test]
async fn blank_inits() {
    let client = OpenAIAccount::new(Opts { 
        cache_filepath: "./src/tests/tmp/blank_cache.json".into(), 
        bill_filepath: "./src/tests/tmp/blank_bill.json".into(),
        ..Default::default()
    }).await.unwrap();

    assert_eq!(
        {
            let mut s = String::new();
            std::fs::File::open("./src/tests/tmp/blank_cache.json").unwrap().read_to_string(&mut s).unwrap();
            s
        },
        "".to_string()
    );
    assert_eq!(
        {
            let mut s = String::new();
            std::fs::File::open("./src/tests/tmp/blank_cache.json").unwrap().read_to_string(&mut s).unwrap();
            s 
        },
        "".to_string()
    );

}