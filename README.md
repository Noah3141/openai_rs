# Rust OpenAI API Wrapper

-   Fork off an old version of [openai-api-rs](https://github.com/dongri/openai-api-rs)
-   Wrapper heavy
-   Ease of use
-   Spending tracking
-   Caching and database storage (with methods to interchange between them)
-   Chat completions on prompts
-   Chat completions on PDFs
-   Chat completions on the cached list of responses (meta completions)

Basically the idea was to be able to easily download a bunch of PDFs, run a prompt on a loop through every PDF in a folder, and then run a prompt on all the responses, for things like automated meta-analyses and topic introductions.

```rust
use openai_rs::*;

let mut client = OpenAIAccount::new(
    GptModel::Gpt4,
    0.2,
    true,
    None,
    None
).await.expect("Initial startup");

client.cache.clear();
client.bill.reset();
client.bill.print();

let mut content = String::new();
let f = std::fs::File::open("inputs.txt")
    .expect("to be able to read the file")
    .read_to_string(&mut content)
    .unwrap();

let cite_blocks: Vec<&str> = content.split("\n").collect();

for block in cite_blocks {
    let query: ChatQuery = client.get_completion(format!("{block}
    type Props = {{
        authors: string[]; // L. M. First
        year: number;
        title: string;
        journal: string;
        edition?: number;
        articleNumber: number;
        pages?: {{
            start: number;
            end: number;
        }};
    }});

    Extract the above information and return the following JSX tag:
    {{ title:   <Citation {{...props}} /> }}").as_str()
    )
    .await
    .expect("success of chat completion");

    if query.from_cache { println!("{} from cache", query.response.id); }
}

let meta_query: MetaQuery = client.meta_complete_cache(
    "Provide a variable declaration of these objects as a single object: const references = { ... };"
    )
    .await
    .expect("success of meta completion");

println!("{}", meta_res.response.choices[0].to_owned().message.content.unwrap());

let res = client.db.insert_query(meta_query.key(), &client.cache).await;


```
