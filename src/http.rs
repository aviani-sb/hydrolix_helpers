use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
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
