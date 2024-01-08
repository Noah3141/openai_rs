use chrono::Utc;
use std::fs;
use sea_orm::{DatabaseConnection, Database, EntityTrait, QueryFilter, ColumnTrait, ActiveValue};
use crate::{
    models::{
        db::{
            chat_completions::ActiveModel as ChatQueryModel, 
            text_completions::ActiveModel as TextQueryModel,
            meta_completions::ActiveModel as MetaQueryModel,
            chat_completions::Column as ChatQueryColumn, 
            text_completions::Column as TextQueryColumn,
            meta_completions::Column as MetaQueryColumn,
            prelude::{
                ChatCompletions,
                MetaCompletions,
                TextCompletions
            }
        }, 
        hash::calculate_hash, cache::Cache, queries::chat_query::Cacheable,
    }, 
    Query,
    ChatQuery,
    TextQuery,
    MetaQuery
};
use std::{error::Error, collections::HashMap};

#[derive(Debug)]
pub struct DbMethods;

/// Methods for coordinating the current cache state and the DB
impl DbMethods {

    pub async fn insert_cache(&self, cache: &Cache) -> std::result::Result<(), Box<dyn Error>> {
        println!("🗄️  Saving cache to database...");
        
        let mut overwritten = false;
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL")?).await?;

        let mut chat_models: Vec<ChatQueryModel> = vec![]; // We will build up lists and then do a single SQL insert for each
        let mut text_models: Vec<TextQueryModel> = vec![];
        let mut meta_models: Vec<MetaQueryModel> = vec![];

        for (cache_key, query) in &cache.entries {
            let query_key_hash = calculate_hash(cache_key);

            match query {
                Query::ChatQuery(query) => {
                    let extant_at_id = ChatCompletions::find().filter(ChatQueryColumn::QueryKeyHash.eq(&query_key_hash)).one(&db).await.expect("Database check for query");

                    if let Some(model) = extant_at_id {
                        ChatCompletions::delete_by_id(model.rid).exec(&db).await.expect("success of deletion by id during insert_cache()");
                        println!("🗄️  Model overwritten at query key hash: {query_key_hash}"); 
                        overwritten = true;
                        let graveyard = std::fs::OpenOptions::new().create(true).append(true).open("graveyard.json").expect("access to graveyard file");
                        serde_json::to_writer_pretty(graveyard, &model).expect("Serialization of an overwritten model to the graveyard");
                    }
                    let model = ChatQueryModel { 
                        timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                        model: ActiveValue::Set(query.model.to_string()), 
                        temperature: ActiveValue::Set(query.temperature as f64), 
                        prompt: ActiveValue::Set(query.prompt.to_string()),
                        query_key: ActiveValue::Set(cache_key.to_string()), 
                        prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                        completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                        total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                        process_time: ActiveValue::Set(query.process_time as i32), 
                        response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                        cost: ActiveValue::Set(query.cost as f64),
                        query_key_hash: ActiveValue::Set(query_key_hash), 
                        rid: ActiveValue::NotSet
                    };

                    chat_models.push(model)
                },
                Query::TextQuery(query) => {
                    let extant_at_id = TextCompletions::find().filter(TextQueryColumn::QueryKeyHash.eq(&query_key_hash)).one(&db).await.expect("Database check for query");

                    if let Some(model) = extant_at_id {
                        TextCompletions::delete_by_id(model.rid).exec(&db).await.expect("success of deletion by id during insert_cache()");
                        println!("🗄️  Model overwritten at query key hash: {query_key_hash}"); 
                        overwritten = true;
                        let graveyard = std::fs::OpenOptions::new().create(true).append(true).open("graveyard.json").expect("access to graveyard file");
                        serde_json::to_writer_pretty(graveyard, &model).expect("Serialization of an overwritten model to the graveyard");
                    }
                    let model = TextQueryModel { 
                        timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                        model: ActiveValue::Set(query.model.to_string()), 
                        temperature: ActiveValue::Set(query.temperature as f64), 
                        prompt: ActiveValue::Set(query.prompt.to_string()),
                        document_title: ActiveValue::Set(query.document_title.to_string()),
                        query_key: ActiveValue::Set(cache_key.to_string()), 
                        prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                        completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                        total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                        process_time: ActiveValue::Set(query.process_time as i32), 
                        response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                        cost: ActiveValue::Set(query.cost as f64),
                        query_key_hash: ActiveValue::Set(query_key_hash), 
                        rid: ActiveValue::NotSet
                    };
                    text_models.push(model)
                },
                Query::MetaQuery(query) => {
                    let extant_at_id = MetaCompletions::find().filter(MetaQueryColumn::QueryKeyHash.eq(&query_key_hash)).one(&db).await.expect("Database check for query");

                    if let Some(model) = extant_at_id {
                        MetaCompletions::delete_by_id(model.rid).exec(&db).await.expect("success of deletion by id during insert_cache()");
                        println!("🗄️  Model overwritten at query key hash: {query_key_hash}"); 
                        overwritten = true;
                        let graveyard = std::fs::OpenOptions::new().create(true).append(true).open("graveyard.json").expect("access to graveyard file");
                        serde_json::to_writer_pretty(graveyard, &model).expect("Serialization of an overwritten model to the graveyard");
                    }
                    let model = MetaQueryModel { 
                        timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                        model: ActiveValue::Set(query.model.to_string()), 
                        temperature: ActiveValue::Set(query.temperature as f64), 
                        prompt: ActiveValue::Set(query.prompt.to_string()),
                        query_key: ActiveValue::Set(cache_key.to_string()), 
                        prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                        completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                        total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                        process_time: ActiveValue::Set(query.process_time as i32), 
                        response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                        cost: ActiveValue::Set(query.cost as f64),
                        query_key_hash: ActiveValue::Set(query_key_hash), 
                        rid: ActiveValue::NotSet
                    };

                    meta_models.push(model)
                },
            }
            
        }
        let _chat_res = ChatCompletions::insert_many(chat_models).exec(&db).await?;
        let _text_res = TextCompletions::insert_many(text_models).exec(&db).await?;
        let _meta_res = MetaCompletions::insert_many(meta_models).exec(&db).await?;

        if overwritten {println!("🪦  Any overwritten models can be recovered in graveyard file.")};
        println!("🗄️  Cache saved to database.");
        Ok(())
    }

    /// Provide the `cache_key` of an cached query, to insert that query into the DB. (The cached entry is not modified)
    pub async fn insert_query(&self, cache_key: String, cache: &Cache) -> Option<i32> {
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("database env var")).await.expect("database connection");
        let query = match cache.entries.get(&cache_key) {Some(s) => s, None => return None};
        
        match query {
            Query::ChatQuery(query) => {
                let model = ChatQueryModel { 
                    timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                    model: ActiveValue::Set(query.model.to_string()), 
                    temperature: ActiveValue::Set(query.temperature as f64), 
                    prompt: ActiveValue::Set(query.prompt.to_string()),
                    query_key: ActiveValue::Set(cache_key.to_string()), 
                    prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                    completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                    total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                    process_time: ActiveValue::Set(query.process_time as i32), 
                    response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                    cost: ActiveValue::Set(query.cost as f64),
                    query_key_hash: ActiveValue::Set(calculate_hash(&cache_key)), 
                    rid: ActiveValue::NotSet
                };
        
                let res = ChatCompletions::insert(model).exec(&db).await.expect("insertion of ActiveModel to db during .insert_query()");
        
                println!("🗄️  Inserted into database query \"{key}\"", key = cache_key);
                Some(res.last_insert_id)
            },
            Query::TextQuery(query) => {
                let model = TextQueryModel { 
                    timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                    model: ActiveValue::Set(query.model.to_string()), 
                    temperature: ActiveValue::Set(query.temperature as f64), 
                    prompt: ActiveValue::Set(query.prompt.to_string()),
                    document_title: ActiveValue::Set(query.document_title.to_string()),
                    query_key: ActiveValue::Set(cache_key.to_string()), 
                    prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                    completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                    total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                    process_time: ActiveValue::Set(query.process_time as i32), 
                    response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                    cost: ActiveValue::Set(query.cost as f64),
                    query_key_hash: ActiveValue::Set(calculate_hash(&cache_key)), 
                    rid: ActiveValue::NotSet
                };
        
                let res = TextCompletions::insert(model).exec(&db).await.expect("insertion of ActiveModel to db during .insert_query()");
        
                println!("🗄️  Inserted into database query \"{key}\"", key = cache_key);
                Some(res.last_insert_id)
            },
            Query::MetaQuery(query) => {
                let model = MetaQueryModel { 
                    timestamp: ActiveValue::Set(Utc::now().naive_local()), 
                    model: ActiveValue::Set(query.model.to_string()), 
                    temperature: ActiveValue::Set(query.temperature as f64), 
                    prompt: ActiveValue::Set(query.prompt.to_string()),
                    query_key: ActiveValue::Set(cache_key.to_string()), 
                    prompt_tokens: ActiveValue::Set(query.response.usage.prompt_tokens), 
                    completion_tokens: ActiveValue::Set(query.response.usage.completion_tokens), 
                    total_tokens: ActiveValue::Set(query.response.usage.total_tokens), 
                    process_time: ActiveValue::Set(query.process_time as i32), 
                    response: ActiveValue::Set(serde_json::to_value(query.response.clone()).expect("conversion to JSON value of query.response")), 
                    cost: ActiveValue::Set(query.cost as f64),
                    query_key_hash: ActiveValue::Set(calculate_hash(&cache_key)), 
                    rid: ActiveValue::NotSet
                };

                let res = MetaCompletions::insert(model).exec(&db).await.expect("insertion of ActiveModel to db during .insert_query()");

                println!("🗄️  Inserted into database query \"{key}\"", key = cache_key);
                Some(res.last_insert_id)
            },
        }
        
    }


    /// Find all in db, convert models to queries, insert queries into the local cache according to query_key, overwriting if `overwrite` is `true` or skipping if not, then overwrite the cache file with the new state of the cache.  Returns the previous state of the cache, before db addition.
    pub async fn read_all_to_cache(&mut self, cache: &mut Cache, overwrite: bool) -> Result<HashMap<String,Query> , Box<dyn Error> > {
        println!("🗄️  Reading database into cache...");
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL")?).await?;
        let previous_state = cache.entries.clone();
        
        let text_models = TextCompletions::find().all(&db).await?;
        let chat_models = ChatCompletions::find().all(&db).await?;
        let meta_models = MetaCompletions::find().all(&db).await?;

        let text_queries: Vec<Query> = text_models.iter().cloned().map(|m| { Query::TextQuery( m.to_query() ) }).collect();
        let chat_queries: Vec<Query> = chat_models.iter().cloned().map(|m| { Query::ChatQuery( m.to_query() ) }).collect();
        let meta_queries: Vec<Query> = meta_models.iter().cloned().map(|m| { Query::MetaQuery( m.to_query() ) }).collect();

        let mut queries: Vec<&Query> = vec![];
        queries.reserve(text_queries.len() + chat_queries.len() + meta_queries.len());

        queries.extend(text_queries.iter());
        queries.extend(chat_queries.iter());
        queries.extend(meta_queries.iter());

        cache.insert_many(queries, overwrite);

        // let cache_file = match fs::OpenOptions::new().create(true).write(true).open(&cache.filepath) {Ok(f)=>f, Err(e)=>panic!("Could not cache query at {}, due to error:  ❌  {}", cache.filepath.display(), e)};
        // serde_json::to_writer_pretty(&cache_file, &cache).expect("Serialization of cache to cache file during read_to_cache");

        println!("🗄️  Database added to cache.");
        Ok(previous_state)
    }

    pub async fn get_text_query(&self, cache_key: String) -> Option<TextQueryModel> {
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("Database env var")).await.expect("Database connection");
        let model = TextCompletions::find().filter(TextQueryColumn::QueryKey.eq(cache_key)).one(&db).await.expect("Database .find() call response success");
        model
    }
    pub async fn get_chat_query(&self, cache_key: String) -> Option<ChatQueryModel> {
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("Database env var")).await.expect("Database connection");
        let model = ChatCompletions::find().filter(TextQueryColumn::QueryKey.eq(cache_key)).one(&db).await.expect("Database .find() call response success");
        model
    }
    pub async fn get_meta_query(&self, cache_key: String) -> Option<MetaQueryModel> {
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("Database env var")).await.expect("Database connection");
        let model = MetaCompletions::find().filter(TextQueryColumn::QueryKey.eq(cache_key)).one(&db).await.expect("Database .find() call response success");
        model
    }
    pub async fn get_query(&self, cache_key: String) -> Option<Query> {
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("Database env var")).await.expect("Database connection");

    }

    pub async fn delete_one_by_id(&self, id: i32) -> Result<(), Box<dyn ErrorTrait>> {
        println!("🗄️  Deleting by id: {id}");
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL")?).await?;
        let _res = QueryCache::delete_by_id(id).exec(&db).await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<(), Box<dyn ErrorTrait>> {
        println!("🗄️  Delete database requested...");
        
        let mut line = String::new();
        println!("🗄️  Press Enter to continue...");
        let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");


        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL")?).await?;
        let _res = QueryCache::delete_many().exec(&db).await?;
        println!("🗄️  Database cleared.\n");
        Ok(())
    }

    pub async fn read_all(&self) -> Result< HashMap<String,Query> , Box<dyn ErrorTrait> > {
        println!("🗄️  Read all from database requested...");
        let db: DatabaseConnection = Database::connect(dotenvy::var("DATABASE_URL").expect("Database env var")).await.expect("Database connection");

        let mut cache: HashMap<String, Query> = HashMap::new();
        let models = QueryCache::find().all(&db).await?;
        for model in models {
            cache.extend([ ( model.query_key.clone(), model.to_query())])
        }

        Ok(cache)
    }

}