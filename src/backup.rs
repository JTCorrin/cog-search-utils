use crate::BackupCommand;
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use reqwest::Client;
use serde_json::Value;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct SearchBody {
    search: String,
    top: usize,
    skip: usize,
}

async fn get_document_count(cmd: &BackupCommand) -> Result<usize, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("api-key", HeaderValue::from_str(&cmd.api_key)?);

    let url = format!("https://{}/indexes/{}/docs/$count?api-version=2020-06-30-Preview", cmd.url, cmd.index);
    let res = client.get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let count: usize = res.parse()?;
    
    Ok(count)
}

async fn backup_worker(cmd: Arc<BackupCommand>, start: usize, end: usize, worker_id: usize) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("api-key", HeaderValue::from_str(&cmd.api_key)?);

    std::fs::create_dir_all(&cmd.file)?;

    for i in (start..end).step_by(cmd.cadence) {
        let url = format!("https://{}/indexes/{}/docs/search.post.search?api-version=2020-06-30-Preview", cmd.url, cmd.index);
        let body = SearchBody {
            search: "*".to_string(),
            top: cmd.cadence,
            skip: i,
        };

        let res = client.post(&url)
            .headers(headers.clone())
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let file_name = format!("{}/backup_{}_{}.json", cmd.file, worker_id, i / cmd.cadence);
        let mut file = File::create(file_name)?;
        write!(file, "{}", res)?;
    }

    Ok(())
}

pub fn run(cmd: BackupCommand) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let count = runtime.block_on(get_document_count(&cmd)).expect("Failed to get document count");

    let cmd_arc = Arc::new(cmd);

    let mut handles = vec![];
    let docs_per_worker = count / cmd_arc.workers;
    for i in 0..cmd_arc.workers {
        let start = i * docs_per_worker;
        let end = if i == cmd_arc.workers - 1 {
            count
        } else {
            start + docs_per_worker
        };
        let cmd_clone = Arc::clone(&cmd_arc);
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(backup_worker(cmd_clone, start, end, i + 1)).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Backing up index {} from {} with {} workers and a cadence of {} documents to file {}. Total documents: {}",
        cmd_arc.index, cmd_arc.url, cmd_arc.workers, cmd_arc.cadence, cmd_arc.file, count
    );
}
