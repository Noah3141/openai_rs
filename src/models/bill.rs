use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
use crate::Query;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bill {
    /// Total cost so far since last `.reset_bill()` in CENTS
    pub(crate) cost: f32,
    /// Total number of prompt tokens (used according to size of prompts) used so far since last `.reset_bill()`
    pub(crate) prompt_tokens: i32,
    /// Total number of completion tokens (used according to size of responses) so far since last `.reset_bill()`
    pub(crate) completion_tokens: i32,
    /// Total token usage so far since last `.reset_bill()`
    pub(crate) total_tokens: i32,
    /// Number of queries recorded so far since last `.reset_bill()`
    pub(crate) query_count: i32,
    /// Number of times a ChatGPT completion was pulled from the cache instead of the API, because the prompt was found in the cache
    pub(crate) cache_retrievals: i32,
    pub(super) filepath: PathBuf,
}

impl Default for Bill {
    fn default() -> Bill {
        Bill {
            cache_retrievals: 0,
            completion_tokens: 0,
            prompt_tokens: 0,
            cost: 0.00,
            query_count: 0,
            total_tokens: 0,
            filepath: "./bill.json".into()
        }
    }
}

impl Bill {

    pub(crate) fn update(&mut self, query: Option<Query>) -> () {

        if let Some(query) = query { 
            let res = query.clone().response();
            self.completion_tokens += res.usage.completion_tokens;
            self.prompt_tokens += res.usage.prompt_tokens;
            self.total_tokens += res.usage.total_tokens;
            self.query_count += 1;
            self.cost += res.cost(&query.model());
        }

        // Save the state of self.bill to file
        let bill = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("Could not update bill at {}, due to error:  ❌  {}", self.filepath.display(), e)};
        serde_json::to_writer_pretty(&bill, &self).expect("Serialization of bill to bill file");
    }

    pub fn reset_bill(&mut self) -> () {
        self.completion_tokens = 0;
        self.prompt_tokens = 0;
        self.total_tokens = 0;
        self.query_count = 0;
        self.cost = 0.00;
        let bill = match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.filepath) {Ok(f)=>f, Err(e)=>panic!("Could not reset bill at {}, due to error:  ❌  {}", self.filepath.display(), e)};
        serde_json::to_writer_pretty(&bill, &self).expect("Serialization of bill to bill file");
        println!("🧾 Bill reset");
    }

    pub fn print(&self) {
        println!("\n");
        println!("🧾 Bill So Far");
        println!("Queries: {}", self.query_count);
        println!("Total Tokens: {}", self.total_tokens);
        println!("Bill: ${:.2}", self.cost / 100.0);
        println!("\n");
    }

}