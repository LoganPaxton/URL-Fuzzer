use futures::stream::{self, StreamExt};
use napi::{Error, Result};
use napi_derive::napi;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Semaphore;

#[napi]
pub async fn fuzz(base_url: String, wordlist_path: String) -> Result<Vec<String>> {
    let file = File::open(&wordlist_path)
        .map_err(|e| Error::from_reason(format!("Failed to open wordlist: {}", e)))?;

    let words: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect();

    let client = Arc::new(
        Client::builder()
            .http2_prior_knowledge()
            .pool_max_idle_per_host(1000)
            .build()
            .map_err(|e| Error::from_reason(format!("Failed to build client: {}", e)))?,
    );

    let max_concurrency = 2000;
    let sem = Arc::new(Semaphore::new(max_concurrency));
    let results = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    stream::iter(words.into_iter())
        .for_each_concurrent(max_concurrency, |word| {
            let url = format!("{}/{}", base_url.trim_end_matches('/'), word);
            let client = client.clone();
            let sem = sem.clone();
            let results = results.clone();

            async move {
                let _permit = sem.acquire().await.unwrap();
                if let Ok(resp) = client.head(&url).send().await {
                    if resp.status() != 404 {
                        let mut res = results.lock().await;
                        res.push(format!("{} -> {}", resp.status(), url));
                    }
                }
            }
        })
        .await;

    let locked = results.lock().await;
    Ok(locked.clone())
}
