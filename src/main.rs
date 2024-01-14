use itertools::Itertools;
use reqwest::{self, StatusCode};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;
use fake_useragent::UserAgents;

const BASE_URL: &str = "https://www.example.co.il/v/73G";
const PARAM_LEN: usize = 3;
const CONCURRENCY_LIMIT: usize = 20;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));
    let user_agents = Arc::new(UserAgents::new());
    let mut handles = vec![];

    // Generate all combinations
    let charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let combinations = charset.chars().permutations(PARAM_LEN);

    for combo in combinations {
        let mut url = String::from(BASE_URL);
        for &ch in &combo {
            url.push(ch);
        }
        let client_clone = client.clone();
        let sem_clone = semaphore.clone();
        let user_agents_clone = Arc::clone(&user_agents);
    
        let handle = task::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();
        
            // Generate a random user agent for each request
            let user_agent = user_agents_clone.random();
            // println!("{user_agent}");
            // return;
        
            // Create a request with the random user agent
            let response = client_clone.get(&url)
                .header(reqwest::header::USER_AGENT, user_agent)
                .send()
                .await;
        
            if let Ok(response) = response {
                if response.status() != StatusCode::NOT_FOUND {
                    println!("URL: {} with status [{}]", url, response.status());
                }
            }
        });
        handles.push(handle);
    }

    println!("starting..");

    // Await all tasks
    for handle in handles {
        handle.await.unwrap();
    }
}