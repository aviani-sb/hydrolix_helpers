use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str;
use tokio::time::Duration;

pub async fn get_data(auth_token: &str, url: &str) -> Result<String, String> {
    let http_client = Client::new();

    // Send the authentication request
    let response = http_client
        .get(url)
        .bearer_auth(auth_token)
        .header("accept", "application/json")
        .header(CONTENT_TYPE, "application/json")
        .timeout(Duration::from_secs(60))
        .send()
        .await;

    let response2 = match response {
        Ok(v) => v, // Capture the response on success
        Err(e) => return Err(format!("{}.{} Error: {e}", file!(), line!(),)),
    };

    // Check if the authentication was successful
    if !response2.status().is_success() {
        return Err(format!(
            "{}.{} Error: {:?}",
            file!(),
            line!(),
            response2.status()
        ));
    }

    match response2.text().await {
        Ok(v) => Ok(v),
        Err(e) => Err(format!(
            "{}.{} Failed to get json data {e}",
            file!(),
            line!()
        )),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResults {
    pub next: u32,
    pub count: u32,
    pub results: Vec<HashMap<String, Value>>,
}

#[async_trait]
pub trait Methods {
    async fn get_data(&self, url: &str, auth_token: &str) -> Result<String, String>;
}

#[derive(Default)]
pub struct Http {}
#[async_trait]
impl Methods for Http {
    async fn get_data(&self, url: &str, auth_token: &str) -> Result<String, String> {
        get_data(url, auth_token).await
    }
}

pub async fn get_paginated(url: &str, auth_token: &str) -> Result<String, String> {
    get_paginated_helper(url, auth_token, &Http::default()).await
}

// The signature mimics `http::get_data`.  The caller will deserialize the array of json themselves, so return a string
pub async fn get_paginated_helper(
    auth_token: &str,
    url: &str,
    methods: &impl Methods,
) -> Result<String, String> {
    let mut page = 1;
    let mut more_results = true;
    let mut results = vec![];
    while more_results {
        let page_url = url.to_owned() + "?page=" + &page.to_string();

        // Send the HTTP GET request to retrieve the list
        let data = match methods.get_data(auth_token, &page_url).await {
            Ok(v) => v,
            Err(e) => return Err(format!("{}.{} Failed to execute: {e}", file!(), line!())),
        };

        // Deserialize the response into the paginated results object
        let response: PaginatedResults = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(_) => {
                // If pagination deserialization fails, the response might _not_ be paginated, so return the raw response and the caller can try to deserialize the array
                return Ok(data);
            }
        };
        for item in response.results {
            results.push(item)
        }
        page += 1;
        if response.next == 0 {
            more_results = false;
        }
    }
    match serde_json::to_string(&results) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}.{} Failed to execute: {e}", file!(), line!())),
    }
}
