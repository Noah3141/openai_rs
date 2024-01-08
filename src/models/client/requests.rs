use reqwest::Response;

use crate::{
    models::{
        client::core::OpenAIAccount, 
        api_error::APIError, 
        ChatCompletionRequest, 
        ChatCompletionResponse
    },
    constants::API_URL_V1,
};


impl OpenAIAccount {

    pub(super) async fn send_completion_request(&self, req: ChatCompletionRequest) -> Result<ChatCompletionResponse, APIError> {
        let res = self.post("/chat/completions", &req).await?;
        let r = res.json::<ChatCompletionResponse>().await;
        match r { Ok(r) => Ok(r), Err(e) => Err(self.new_error(e)) }
    }

    fn new_error(&self, err: reqwest::Error) -> APIError {
        APIError { message: err.to_string() }
    }

    pub async fn post<T: serde::ser::Serialize>(&self, path: &str, params: &T) -> Result<Response, APIError> {
        let client = reqwest::Client::new();
        let url = format!("{API_URL_V1}{path}");
        let res = client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &self.api_key)
            .json(&params)
            .send()
            .await;
        match res {
            Ok(res) => match res.status().is_success() { true => Ok(res), false => Err(APIError { message: format!(  "{}: {}", res.status(), res.text().await.unwrap()   ) })  }, 
            Err(e) => Err(self.new_error(e)),
        }
    }
    
}