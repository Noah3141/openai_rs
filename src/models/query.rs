use serde::{Serialize, Deserialize};

use crate::{
    models::db::*,
    GptModel
};
use super::{
    queries::chat_query::Cacheable, 
    ChatCompletionResponse,
    ChatQuery, 
    TextQuery, 
    MetaQuery, 
};

/// The type of request response that occured for this query. A prompt completion involved Chat Completion from a prompt, whereas a PDF summary is generated from PDF. <br>
/// The key in the cache for a prompt completion is the prompt, whereas the key for a PdfCompletion is the pdf's filename, which should always match its storage name on disc.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Query {
    ChatQuery(ChatQuery),
    TextQuery(TextQuery),
    MetaQuery(MetaQuery),
}

impl Query {
    pub fn response(self) -> ChatCompletionResponse {
        match self {
            Query::ChatQuery(q) => q.response,
            Query::TextQuery(q) => q.response,
            Query::MetaQuery(q) => q.response,
        }
    }

    pub fn model(self) -> GptModel {
        match self {
            Query::ChatQuery(q) => q.model,
            Query::TextQuery(q) => q.model,
            Query::MetaQuery(q) => q.model,
        }
    }

    pub fn expect_as_chat(self) -> ChatQuery {
        if let Query::ChatQuery(query) = self {query} else {panic!("Expected to be a ChatQuery {self:#?}")}
    }
    pub fn expect_as_text(self) -> TextQuery {
        if let Query::TextQuery(query) = self {query} else {panic!("Expected to be a TextQuery {self:#?}")}
    }
    pub fn expect_as_meta(self) -> MetaQuery {
        if let Query::MetaQuery(query) = self {query} else {panic!("Expected to be a MetaQuery {self:#?}")}
    }


}


impl Cacheable for Query {
    fn key(&self) -> String {
        match self {
            Query::ChatQuery(query) => query.key(),
            Query::TextQuery(query) => query.key(),
            Query::MetaQuery(query) => query.key(),
        }
    }
}