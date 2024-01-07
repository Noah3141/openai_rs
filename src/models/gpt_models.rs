
use {
    serde::{Serialize,Deserialize},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize)]
pub enum GptModel {
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo0613,
    Gpt4,
    Gpt40314,
    Gpt432k,
    Gpt432k0314,
    Gpt40613,
}

impl Serialize for GptModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}


impl ToString for GptModel {
    #[inline]
    /// Converts the enum variant to a String corresponding to that model's string in the OpenAI documentation
    /// 
    /// # Example
    ///  
    /// `assert_eq!( GptModels::Gpt4.to_string() , "gpt-4" )`
    fn to_string(&self) -> String {
        use GptModel::*;
        use crate::constants::model_strings::*;
        match self {
            Gpt35Turbo => GPT3_5_TURBO,
            Gpt35Turbo0613 => GPT3_5_TURBO_0613,

            Gpt35Turbo16k => GPT3_5_TURBO_16K,
            
            Gpt4 => GPT4,
            Gpt40613 => GPT4_0613,
            Gpt40314 => GPT4_0314,
            
            Gpt432k => GPT4_32K,
            Gpt432k0314 => GPT4_32K_0314,
        }.to_string()
    }
}

impl GptModel {

    pub fn from_string(model: &String) -> GptModel {
        GptModel::from_str( model.as_str() )
    }

    pub fn from_str(model: &str) -> GptModel {
        use GptModel::*;
        use crate::constants::model_strings::*;

        match model {
            GPT3_5_TURBO => Gpt35Turbo,
            GPT3_5_TURBO_16K => Gpt35Turbo16k,
            GPT3_5_TURBO_0613 => Gpt35Turbo0613,
            GPT4 => Gpt4,
            GPT4_0314 => Gpt40314,
            GPT4_32K => Gpt432k,
            GPT4_32K_0314 => Gpt432k0314,
            GPT4_0613 => Gpt40613,
            &_ => panic!("A new model needs added or else `GptModel::from_string(string)` was called with an invalid model string: {model}")
        }
    }
}

