use crate::DocumentCommand;
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub fn run(cmd: DocumentCommand) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let document = runtime.block_on(get_document(&cmd)).expect("Failed to get document");

    println!("Document: {:?}", document);
}

async fn get_document(cmd: &DocumentCommand) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("api-key", HeaderValue::from_str(&cmd.api_key)?);

    let url = format!("https://{}/indexes/{}/docs/{}?$select={}&api-version=2020-06-30-Preview", 
                      cmd.url, cmd.index, cmd.key, cmd.select);

    let res = client.get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}
