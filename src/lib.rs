#[cfg(test)]
mod basic {
    use crate::models::ChatCompletionRequest;

    use super::*;

    #[test]
    fn initializes() {
        let client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, None, None);
    }

    #[tokio::test]
    async fn manual_prompt() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, None, None);
        
        let res = client.get_completion(&"What's the deal with airplane food?".to_string()).await;
        let query = res.expect("manual_prompt");

        dbg!("{:#?}", query);
    }

    #[tokio::test]
    async fn caching_retrieves_cache_when_present() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, None, None);
        
        let prompt = "What's the deal with airplane food?".to_string();

        let res = client.get_completion(&prompt).await;
        let query = res.expect("manual_prompt");

        dbg!("{:#?}", query);

        let res = client.get_completion(&prompt).await;
        let requery = res.expect("manual_prompt");

        assert_eq!(requery.from_cache, true)

    }


    #[tokio::test]
    async fn database_behavior() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, None, None);
        
        let prompt = "What's the deal with airplane food?".to_string();

        client.apply_prompt_to_pdf(
            "", &"", &""
        );
    }


}

pub mod models;
pub mod client;

pub mod constants;

pub use client::OpenAIAccount;
pub use models::GptModel;
pub use models::Query;