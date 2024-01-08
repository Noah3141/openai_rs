use crate::GptModel;

use super::{req_and_res::ChatCompletionMessage};

use {
    serde::{Serialize,Deserialize},
    std::collections::HashMap
};

#[derive(Debug, Serialize, PartialEq)]
pub struct ChatCompletionRequest {
    pub model: GptModel,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}

impl Default for ChatCompletionRequest {
    /// Applies:
    /// ```
    /// use openai_rs::models::{request::ChatCompletionRequest, gpt_models::GptModel};
    /// 
    /// let default = ChatCompletionRequest { ..Default::default() };
    /// 
    /// assert_eq!(default.function_call, None);
    /// assert_eq!(default.functions, None);
    /// assert_eq!(default.model, GptModel::Gpt35Turbo);
    /// assert_eq!(default.temperature, None);
    /// assert_eq!(default.messages, vec![]);
    /// ```
    fn default() -> Self {
        Self {
            function_call: None,
            functions: None,
            model: GptModel::Gpt35Turbo,
            temperature: None,
            messages: vec![]
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessage,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FunctionParameters>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JSONSchemaType {
    Object,
    Number,
    String,
    Array,
    Null,
    Boolean,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct JSONSchemaDefine {
    #[serde(rename = "type")]
    pub schema_type: Option<JSONSchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JSONSchemaDefine>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub schema_type: JSONSchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum FinishReason {
    stop,
    length,
    function_call,
    content_filter,
    null,
}
