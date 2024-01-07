use super::prelude::*
use crate::models::*;


// impl Model {
//     pub fn to_query(&self) -> Query {
//         Query { 
//             prompt: self.prompt.clone(), 
//             cost: self.cost, 
//             response: serde_json::from_value(self.response.clone()).unwrap(), 
//             process_time: self.process_time as u64, 
//             model: GptModel::from_string(&self.model), 
//             temperature: self.temperature,
//             from_cache: true, 
//         }

//     }

// }