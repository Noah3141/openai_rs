CREATE TABLE chat_completions (
    rid serial PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    model varchar(45) NOT NULL,
    temperature float NOT NULL,
    prompt TEXT NOT NULL,
    query_key text NOT NULL,
    query_key_hash char(64) NOT NULL,
    prompt_tokens int NOT NULL,
    completion_tokens int NOT NULL,
    total_tokens int NOT NULL,
    process_time int NOT NULL,
    response jsonb NOT NULL,
    cost float NOT NULL,
    CONSTRAINT query_key_hash_unique UNIQUE (query_key_hash)
);

/* 

sea-orm-cli generate entity -o /src/models/db --with-serde both 

"CLI create files for table at Output=src/models/ and be sure to add serialize-deserialize implementations for it all"


Uses DATABASE_URL by default, use --help flag with any combination of commands to get the next layer's flags
This is Rust: extra effort goes into ENSURING a value is not accidentally null, therefore we can easily say no to the need to skip "not null", 
and we therefore also WANT to include NOT NULL, so that we aren't literally-actually uselessly checking all our data a "options", reduntantly checking
if it's null, when we already did it long long ago.
*/

