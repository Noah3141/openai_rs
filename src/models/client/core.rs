use std::{
    io,
    fs,
    collections::HashMap, 
    path::PathBuf
};
use sea_orm::{DatabaseConnection, Database};

use crate::{
    models::{
        client::database::DbMethods,
        cache::Cache, 
        Bill, 
    },
    GptModel, 
    Query, 
};


#[derive(Debug)]
pub struct OpenAIAccount  { 
    /// Choose from models::gpt_models From this
    pub(super) model: GptModel,
    /// Default value looks for `CHATGPT_API_KEY` environment var
    pub(super) api_key: String,
    pub(super) temperature: f32,
    /// Attribute used to save and retrieve running metrics, which are running totals of Query metrics. 
    /// <br> This variable is serialized into and deserialized from this OpenAIAccount's `.self.cache_filepath` attribute. The running total can be reset with ...
    /// <br> See struct `Bill` for a list of what is tracked.
    pub bill: Bill,
    /// Attribute used to save and retrieve Query metrics. 
    /// This variable is serialized into and deserialized from self.cache_filepath constant.
    /// If a query completion is sent, and the prompt is already found in the cache, the cached response is retrieved, and a new API request is not sent.
    /// Keys are prompts, values are Queries (which themselves hold the prompt, model, etc.)
    pub cache: Cache,
    pub db: DbMethods,
}

pub struct Opts {
    pub model: GptModel,
    /// `0.0 - 0.4`: Produces more focused, conservative, and consistent responses. <br> `0.5 - 0.7`: Strikes a balance between creativity and consistency. <br> `0.8 - 1.0`: Generates more creative, diverse, and unexpected outputs. <br> Default sets to 0.0
    pub temperature: f32,
    /// `bool` for whether you plan to use database functionality. If not, it will not try to connect, and panic on usage of db methods. Else if true, connects once on initialization.
    pub database: bool,
    /// If path does not exist, error. Will not create path for you.
    pub bill_filepath: PathBuf, 
    /// If path does not exist, error. Will not create path for you.
    pub cache_filepath: PathBuf 
}

impl Default for Opts {
    /// Defaults:
    /// ```
    /// use openai_rs::*;
    /// 
    /// Opts {
    ///     model: GptModel::Gpt35Turbo,
    ///     temperature: 0.5,
    ///     database: false,
    ///     bill_filepath: "./bill.json".into(),
    ///     cache_filepath: "./cache.json".into() 
    /// };
    /// ```
    fn default() -> Self {
        Opts {
            model: GptModel::Gpt35Turbo,
            temperature: 0.5,
            database: false,
            bill_filepath: "./bill.json".into(),
            cache_filepath: "./cache.json".into() 
        }
    }
}


impl Default for OpenAIAccount {
    fn default() -> OpenAIAccount {
        OpenAIAccount {
            api_key: dotenvy::var("CHATGPT_API_KEY").unwrap().to_string(),
            temperature: 0.0,
            db: DbMethods { conn: None },
            cache: Cache { ..Default::default() },
            bill: Bill { ..Default::default() },
            model: GptModel::Gpt35Turbo16k,
        }
    }
}

impl OpenAIAccount {
    pub async fn new(opts: Opts) -> Result<OpenAIAccount, Status> {

        let bill_filepath = opts.bill_filepath;
        let cache_filepath = opts.cache_filepath;

        let api_key = dotenvy::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY environment variable").to_string();
        
        let mut res = Ok(());
        let db = DbMethods::try_init().await.map_err(|e| { res = Err(e) }).ok();

        match db {
            None => match opts.database {
                true => return Err(res.unwrap_err()),
                false => (),
            },
            _ => ()
        }

        let bill = match fs::File::open(&bill_filepath) {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                // Read the JSON contents of the file as an instance of...
                let bill: Bill = serde_json::from_reader(reader).unwrap_or_else(|e| {
                    println!("🧾 Initializing client with default blank bill due to:  ❌  {e}") ; 
                    Bill { filepath: bill_filepath, ..Default::default() }
                });
                println!("🧾 Bill read from: {}", bill.filepath.display());
                bill
            },
            Err(_) => {
                fs::File::create(&bill_filepath).expect(format!("Tried but failed to create a new bill file, after having not being able to open: {}", bill_filepath.display()).as_str() );
                let bill = Bill { filepath: bill_filepath, ..Default::default() };
                println!("🧾 Empty Bill created at: {}", bill.filepath.display());
                bill
            },
        };

        // Read the cache into memory or else initialize empty
        let cache: Cache = match fs::File::open(&cache_filepath) {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                let entries: HashMap<String, Query> = serde_json::from_reader(reader).unwrap_or_else(|e| { 
                    if let serde_json::error::Category::Eof = e.classify() { HashMap::new() } else { println!("🗳️ Initializing client with blank cache due to:  ❌  {e}"); HashMap::new() }  
                });
                let cache = Cache {
                    entries,
                    filepath: cache_filepath
                };
                println!("🗳️   Cache read from: {}", &cache.filepath.display());
                cache
            },
            Err(_) => { // HashMap<String, Query>
                fs::File::create(&cache_filepath).expect(format!("Tried but failed to create a new cache file, after having not being able to open:  {}", cache_filepath.display()).as_str() );
                let blank_cache = Cache {
                    filepath: cache_filepath,
                    ..Default::default()
                };
                println!("🗳️   Empty Cache created at: {}", blank_cache.filepath.display());
                blank_cache
            },
        };

        let _graveyard = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open("graveyard.json").expect("access to graveyard file");
        println!("🪦  Graveyard backups cleared.");

        println!("🌡️   Model initialized at temperature {}", opts.temperature);
        Ok(OpenAIAccount {
            bill,
            cache,
            api_key,
            model: opts.model,
            temperature: opts.temperature,
            db: DbMethods {
                conn: db
            },
            ..Default::default()
        })
    }

    
    pub fn set_temperature(&mut self, temperature: f32) { 
        if self.temperature < temperature {println!("🌡️  Temperature raised to {temperature}")} else {println!("🌡️  Temperature lowered to {temperature}")}
        self.temperature = temperature; 
    }
}

#[derive(Debug)]
pub enum Status {
    Success,
    Error(String),
    NotFoundError,
    OpenAIError,
    APIReachedLimit,
    RetrievedUnexpectedQueryType
}