use serde::{Serialize, Deserialize};

use crate::Query;
use std::{
    path::PathBuf,
    collections::HashMap,
    fs, 
};

use super::queries::chat_query::Cacheable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cache {
    pub entries: HashMap<String,Query>,
    pub filepath: PathBuf,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            entries: HashMap::new(),
            filepath: "./cache.json".into()
        }
    }
}

impl Cache {
    /// Resets both the cache file and in-memory cache to empty
    pub fn clear(&mut self) {
        self.entries.clear();
        let cache_file = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("🗳️   clear() had trouble initializing a new blank cache file at '{}' : \n❌  {}", self.filepath.display(), e)};
        serde_json::to_writer_pretty(&cache_file, &self.entries).expect("Serialization of cache to cache file");
        println!("🗳️   Cache cleared at: {}", self.filepath.display());
    }

    pub fn remove(&mut self, cache_key: String) -> Option<(String, Query)> {
        let entry = self.entries.remove_entry(&cache_key);
        

        match entry {
            Some(entry) => {
                println!("🗳️   Removed cache entry at key: \"{cache_key}\"");
                // Update the cache file
                let cache_file = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("🗳️   Could not re-write cache file after removal at {}, due to error:  ❌  {}", self.filepath.display(), e)};
                serde_json::to_writer_pretty(&cache_file, &self.entries).expect("Serialization of cache to cache file");
                Some(entry)
            },
            None => None
        }
    }

    pub(super) fn insert(&mut self, query: &Query) -> () {

        let cache_key = query.key();

        match self.entries.insert(cache_key, query.clone()) {None => (), Some(query)=> { 
            let graveyard = std::fs::OpenOptions::new().create(true).append(true).open("graveyard.json").expect("access to graveyard file");
            serde_json::to_writer_pretty(graveyard, &query).expect("Serialization of an overwritten model to the graveyard");
            println!("\n\n");
            println!("🗳️   Caching a query resulted in an overwrite."); 
            println!("🪦   The overwritten query can be found in the graveyard file.");
        }};
        // Save the state of self.cache to file
        let cache = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("🗳️   Could not cache query at {}, due to error:  ❌  {}", self.filepath.display(), e)};
        serde_json::to_writer_pretty(&cache, &self.entries).expect("Serialization of cache to cache file");
    }

    pub(super) fn insert_many(&mut self, queries: Vec<&Query>, overwrite: bool) -> () {
        for query in queries {
            let query_key = query.key();
            match overwrite {
                true => {
                    match self.entries.insert(query_key, query.clone()) {None => (), Some(query) => { 
                        let graveyard = std::fs::OpenOptions::new().create(true).append(true).open("graveyard.json").expect("access to graveyard file");
                        serde_json::to_writer_pretty(graveyard, &query).expect("Serialization of an overwritten model to the graveyard");
                        println!("\n\n");
                        println!("🗳️   Caching a query resulted in an overwrite."); 
                        println!("🪦   The overwritten query can be found in the graveyard file.");
                    }};
                },
                false => {
                    match self.entries.get(&query_key) {
                        Some(_) => (),
                        None => {
                            self.entries.insert(query_key, query.clone());
                        },
                    }
                },
            }
        }

        let cache = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("🗳️   Could not cache query at {}, due to error:  ❌  {}", self.filepath.display(), e)};
        serde_json::to_writer_pretty(&cache, &self.entries).expect("Serialization of cache to cache file");
    }
}