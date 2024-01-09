pub mod models;
pub mod constants;

pub use crate::{
    models::{
        client::{
            core::{
                OpenAIAccount
            },
        },
        queries:: *,
        GptModel,
        Query,
    }
};


#[cfg(test)]
mod basic {

    use crate::OpenAIAccount;
    use crate::GptModel;
    use crate::Query;
    use crate::chat_query::Cacheable;

    #[tokio::test]
    async fn initializes() {
        let client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, false, None, None).await.unwrap();
    }

    #[tokio::test]
    async fn manual_prompt() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, false, None, None).await.unwrap();
        
        let res = client.get_completion("What's the deal with airplane food?").await;
        let query = res.expect("manual_prompt");

        dbg!("{:#?}", query);
    }

    fn initializes_at_provided_locations() {}

    #[tokio::test]
    async fn caching_retrieves_cache_when_present() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, false, None, None).await.unwrap();
        
        let prompt = "What's the deal with airplane food?";

        let res = client.get_completion(prompt).await;
        let query = res.expect("manual_prompt");

        let res = client.get_completion(prompt).await;
        let requery = res.expect("manual_prompt");

        assert_eq!(requery.from_cache, true)

    }


    #[tokio::test]
    async fn database_insert_and_read() {
        let mut client = OpenAIAccount::new(GptModel::Gpt35Turbo, 0.5, true, None, None).await.unwrap();
        client.cache.clear();
        let prompt = "What's the deal with airplane food?";
        let query = client.get_completion(prompt).await.unwrap();
        
        assert!(!query.from_cache); // Not from cache
        assert!(client.cache.entries.get(&query.key()).is_some()); // But got put in
        
        let res = client.db.insert_query(query.key(), &client.cache).await.unwrap(); // I now put into DB

        client.cache.clear(); // Wipe cache
        
        assert!(client.cache.entries.get(&query.key()).is_none()); // Find nothing in cache at its key
        assert!(client.db.get_chat_query(query.key()).await.is_some()); // But find it in the db at its key

        let res = client.db.read_all_to_cache(&mut client.cache, false).await; // I take db and put to cache

        assert!(res.unwrap().get(&query.key()).is_none()); // Old state should lack the db read
        
        assert_eq!(
            client.cache.entries.get(&query.key()).unwrap().to_owned().expect_as_chat().process_time,
            Query::ChatQuery(query.clone()).expect_as_chat().process_time, 
        );
        
        assert_eq!(
            client.cache.entries.get(&query.key()).unwrap().to_owned().expect_as_chat().response.id, // And the cache at that key is the SAME THING
            Query::ChatQuery(query).expect_as_chat().response.id, // as the old query
        )


    }

    #[tokio::test]
    async fn basic_pdf() {
        let jsx_prompt = "
        type Props = {
            authors: string[]; // L. M. First
            year: number;
            title: string;
            journal: string;
            edition?: number;
            articleNumber: number;
            pages?: {
                start: number;
                end: number;
            };
            doiLink: string;
        };
        Extract the above information and return the following JSX tag:
        <Citation {props} />
        ";

        let mut client = OpenAIAccount::new(GptModel::Gpt4, 0.2, false, None, None).await.expect("");

        let res = client.apply_prompt_to_pdf("Is Higher Better", jsx_prompt, None)
            .await
            .unwrap();
        

    }


    #[tokio::test]
    async fn basic_parser() {
        let jsx_prompt = "
        Xi, C. H. E. N., Hao, C. H. E. N., LU, W. B., Zhang, M., Yuan, T. A. O., Wang, Q. C., ... & Zhang, W. B. (2023). Lower Baseline LDL Cholesterol Affects All-cause Mortality in Patients with First Percutaneous Coronary Intervention. Biomedical and Environmental Sciences, 36(4), 324-333

        type Props = {
            authors: string[]; // L. M. First
            year: number;
            title: string;
            journal: string;
            edition?: number;
            articleNumber: number;
            pages?: {
                start: number;
                end: number;
            };
            doiLink: string;
        };  
        Extract the above information and return the following JSX tag:
        { {title}:   <Citation {...props} /> }
        ";

        let mut client = OpenAIAccount::new(GptModel::Gpt4, 0.2, false, None, None).await.expect("");

        let res = client.get_completion(jsx_prompt)
            .await
            .unwrap();
        
        println!("{}", res.response.choices[0].message.content.as_ref().unwrap());

    }


}

