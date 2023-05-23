use crate::RestoreCommand;
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::Path;

async fn restore_worker(cmd: &RestoreCommand, file_name: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("api-key", HeaderValue::from_str(&cmd.api_key)?);

    // Assuming your backups are JSON and structured as expected by the IndexBatch API
    let backup_file = fs::read_to_string(file_name)?;
    let documents: Value = serde_json::from_str(&backup_file)?;

    let url = format!("https://{}/indexes/{}/docs/index?api-version=2020-06-30-Preview", cmd.url, cmd.index);
    let body = json!({
        "value": documents
    });

    let res = client.post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Successfully restored from {}", file_name);
    } else {
        println!("Failed to restore from {}. Status: {}", file_name, res.status());
    }

    Ok(())
}

pub fn run(cmd: RestoreCommand) {
    println!("Restoring index {} from {} using backup files from {}",
        cmd.index, cmd.url, cmd.file);

    // Get a list of backup files
    let entries = fs::read_dir(&cmd.file).expect("Failed to read directory");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let file_name = path.to_str().expect("Failed to convert path to string").to_string();
            runtime.block_on(restore_worker(&cmd, &file_name)).expect("Failed to restore worker");
        }
    }
}
