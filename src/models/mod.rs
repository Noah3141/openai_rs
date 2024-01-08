pub mod gpt_models;
pub mod request;
pub mod response;
pub mod req_and_res;
pub mod api_error;
pub mod bill;
pub mod query;
pub mod db;
pub mod hash;
pub mod queries;
pub mod client;
pub mod cache;

// Hoist up these structs into the "::models::{}" scope, out from their individual files (they are still available there too)
pub use req_and_res::ChatCompletionMessage;
pub use req_and_res::MessageRole;
pub use request::ChatCompletionRequest;
pub use response::ChatCompletionResponse;
pub use bill::Bill;
pub use queries::{
    meta_query::MetaQuery, 
    text_query::TextQuery, 
    chat_query::ChatQuery
};
pub use query::Query;
pub use gpt_models::GptModel;