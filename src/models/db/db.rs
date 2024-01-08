use super::*;
use crate::models::queries::*;
use crate::GptModel;

impl chat_completions::Model {
    pub fn to_query(self) -> ChatQuery {
        ChatQuery { 
            prompt: self.prompt.clone(), 
            cost: self.cost as f32, 
            response: serde_json::from_value(self.response).unwrap(), 
            process_time: self.process_time as u64, 
            model: GptModel::from_string(&self.model), 
            temperature: self.temperature as f32,
            from_cache: true, 
        }
    }
}

impl text_completions::Model {
    pub fn to_query(self) -> TextQuery {
        TextQuery { 
            prompt: self.prompt,
            document_title: self.document_title,
            response: serde_json::from_value(self.response).unwrap(), 
            process_time: self.process_time as u64, 
            model: GptModel::from_string(&self.model), 
            temperature: self.temperature as f32,
            cost: self.cost as f32, 
            from_cache: true, 
        }
    }
}

impl meta_completions::Model {
    pub fn to_query(self) -> MetaQuery {
        MetaQuery { 
            prompt: self.prompt, 
            cost: self.cost as f32, 
            response: serde_json::from_value(self.response).unwrap(), 
            process_time: self.process_time as u64, 
            model: GptModel::from_string(&self.model), 
            temperature: self.temperature as f32,
            from_cache: true, 
        }
    }
}