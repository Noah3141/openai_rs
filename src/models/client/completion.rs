use crate::{
    models::{
        client::core::{OpenAIAccount, Status},
        ChatCompletionRequest, 
        ChatCompletionMessage, 
        MessageRole,
        ChatQuery, 
        TextQuery,
        MetaQuery,
    }, 
    constants::{pdf_path::DEFAULT_PDF_DIR},
    Query,
};




impl OpenAIAccount {
        /// Sends the prompt as the first message, and returns the chat completion response.
    /// <br> Checks cache for presence of prompt, and returns the cache value if present instead of repeating request.
    /// <br> Inputting a model will use that model, otherwise `None` will default to the model used in the .new() initiator.
    pub async fn get_completion(&mut self, prompt: &str) -> Result<ChatQuery, Status> {

        let model = self.model;
        let key = ChatQuery::key(prompt);

        let query = match self.cache.entries.get(&key) {
            // If found in cache, retrieve the query
            Some(query) => {
                if let Query::ChatQuery(pq) = query {
                    let mut pq = pq.clone(); 
                    pq.from_cache = true;
                    self.bill.cache_retrievals += 1; 
                    self.bill.update(None); 
                    println!("--[Cached Answer]--");
                    pq
                } else {
                    return Err(Status::RetrievedUnexpectedQueryType)
                }
            },
            // If absent, send to OpenAI
            None => {
                let from_cache = false;
                let req = ChatCompletionRequest {
                    model: model,
                    messages: vec![ChatCompletionMessage {
                        role: MessageRole::user,
                        content: Some(prompt.to_string()),
                        name: None,
                        function_call: None,
                    }],
                    temperature: Some(self.temperature.into()),
                    ..Default::default()
                };

                let start_time = std::time::Instant::now();
                let response = match self.send_completion_request(req).await {Ok(res) => res, Err(e) => return Err(Status::Error(e.to_string()))};
                let process_time = start_time.elapsed().as_secs();

                let query = ChatQuery {prompt: prompt.to_string(), response: response.clone(), cost: response.cost(&model), process_time, model, temperature: self.temperature, from_cache };
                
                self.cache.insert(&Query::ChatQuery( query.clone() ));
                self.bill.update(Some(Query::ChatQuery( query.clone() )));

                println!("--[Bill so far: ${:.2}]--", self.bill.cost / 100.0);
                println!("--[Took: {}, Cost: ¢{:.4}]--", process_time, (response.cost(&model)));
                query
            },
        };

        Ok(query)


    }

    pub async fn apply_prompt_to_pdf(&mut self, pdf_title: &str, prompt: &str, input_dir: Option<String>) -> Result<TextQuery, Status> {
        println!("\n--🗳️");
        let dir = match input_dir { None => DEFAULT_PDF_DIR.to_string(), Some(s) => s };
        let path_to_pdf = if dir.ends_with("/") {format!("{dir}{pdf_title}.pdf")} else if dir.contains("\\") {format!("{dir}\\{pdf_title}.pdf")} else {format!("{dir}/{pdf_title}.pdf")};
        let key = TextQuery::key(prompt, pdf_title);

        let query = match self.cache.entries.get(&key) {
            // If found in cache, retrieve the query
            Some(query) => {
                let mut query = query.clone().expect_as_text();
                query.from_cache = true; 
                self.bill.cache_retrievals += 1; 
                self.bill.update(None); 
                println!("--[Cached Answer]--");
                query
            },
            // If absent, send to OpenAI
            None => {
                let from_cache = false;
                println!("--[Sending to GPT]--");
                // Load the pdf from the provided file path, or else return to the caller a NotFoundError 
                let pdf = lopdf::Document::load(path_to_pdf).map_err(|e| return Status::Error(e.to_string()))?;
            
                let mut doc = String::new();
                for page in 1..=pdf.get_pages().len() {
                    let content = pdf.extract_text(&[page as u32]).expect("parse");
                    doc.push_str(&content);
                }
                
                let req = ChatCompletionRequest {
                    model: self.model,
                    temperature: Some(self.temperature.into()),
                    messages: vec![
                        ChatCompletionMessage {
                            role: MessageRole::system,
                            content: Some(String::from("You will receive a document, and a prompt regarding the document.")),
                            ..Default::default()
                        },
                        ChatCompletionMessage {
                            role: MessageRole::assistant,
                            content: Some(format!("{doc}")),
                            ..Default::default()
                        },
                        ChatCompletionMessage {
                            role: MessageRole::user,
                            content: Some(format!("{prompt}")),
                            ..Default::default()
                        },
                    ], 
                    ..Default::default()
                };

                let start_time = std::time::Instant::now();
                let response = self.send_completion_request(req).await.map_err(|e| Status::Error(e.to_string()))?;
                let process_time = start_time.elapsed().as_millis() as u64;
                
                println!("--[Completion received]--");

                // Build Query from Response
                let text_query = TextQuery { prompt: prompt.to_string(), response: response.clone(), document_title: pdf_title.to_string(), model: self.model, process_time, cost: response.cost(&self.model), temperature: self.temperature, from_cache };
                let query_for_cache = Query::TextQuery(text_query.clone());
                
                self.cache.insert(&query_for_cache); // Add Query to Cache
                self.bill.update(Some(query_for_cache)); // Add data to Bill
                println!("--[Bill now shows: ${:.2}]--", self.bill.cost / 100.0);
                println!("--[Took: {}ms, Cost: ¢{:.4}]--", process_time, (response.cost(&self.model)));
                text_query
            },
        };
        println!("--[Got from or created to cache ('{}') under key: \"{key}\"]--", self.cache.filepath.display());
        println!("--");
        Ok(query)
    }


    pub async fn meta_complete_cache(&mut self, prompt: &str) -> Result<MetaQuery, Status>  {
        println!("\n--🗳️  Meta Completion");
        
        let key = MetaQuery::key(&prompt);
        
        let query = {
                let from_cache = false;
                // Convert the cache's PdfCompletions into a list of responses
                let mut build_response_list = String::new();
                let mut iter = 0;
                println!("--[Combining Essays:");
                for (_cache_key, query) in &self.cache.entries {
                    if let Query::TextQuery(query) = query {
                        iter += 1;
                        build_response_list.push_str(format!("\n\n{iter})\n").as_str());
                        let content = query.response.choices[0].message.content.clone().expect("presence of content field in GPT-response");
                        build_response_list.push_str(content.as_str());
                    }
                    //if iter == 3 {println!("--Current state of the input at 3:\n{build_input}");}
                }
                let response_list = build_response_list;
                println!("\n--Essays combined and ready for meta-completion.]--");

                println!("--[Sending to GPT]--");
                let req = ChatCompletionRequest {
                    model: self.model,
                    messages: vec![
                        ChatCompletionMessage {
                            content: Some(String::from("You will receive a prompt, and data to which you will apply that prompt.")),
                            ..Default::default()
                        },
                        ChatCompletionMessage {
                            role: MessageRole::assistant,
                            content: Some(format!("Data: {response_list}")),
                            ..Default::default()
                        },
                        ChatCompletionMessage {
                            role: MessageRole::user,
                            content: Some(format!("Prompt: {prompt}")),
                            ..Default::default()
                        },
                    ],..Default::default()
                };

                let start_time = std::time::Instant::now();
                let response = self.send_completion_request(req).await.map_err(|e| Status::Error(e.to_string()))?;
                let process_time = start_time.elapsed().as_millis() as u64;
                
                println!("--[Completion received]--");

                let meta_query = MetaQuery { prompt: prompt.to_string(), response: response.clone(), model: self.model, process_time, cost: response.cost(&self.model), temperature: self.temperature, from_cache };
                let query_for_cache = Query::MetaQuery(meta_query.clone());

                self.cache.insert(&query_for_cache);
                self.bill.update(Some(query_for_cache));
                
                println!("--[Bill now shows: ${:.2}]--", self.bill.cost / 100.0);
                println!("--[Took: {}ms, Cost: ¢{:.4}]--", process_time, (response.cost(&self.model)));

                meta_query
        };

        println!("--[Created meta completion query to cache under key: \"{key}\"]--");
        println!("--");
        Ok(query)

    }
}