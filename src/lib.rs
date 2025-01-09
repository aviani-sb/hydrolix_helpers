use once_cell::sync::Lazy;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HydrolixToken {
    pub value: String,
    pub org_list: Vec<String>,
    pub expires_at: Instant,
    pub hits: usize,
}

#[allow(dead_code)]
static TOKEN_CACHE: Lazy<Mutex<HydrolixToken>> = Lazy::new(|| {
    Mutex::new(HydrolixToken {
        value: "".to_string(),
        org_list: vec![],
        expires_at: Instant::now(),
        hits: 0,
    })
});

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HydrolixAuth {
    base_url: String,
    username: String,
    password: String,
    http_client: Client,
    token: HydrolixToken,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AuthToken {
    access_token: String,
    expires_in: Option<u64>, // Optional in case the field is missing
    token_type: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Org {
    uuid: String,
    name: String,
    cloud: String,
    #[serde(default)]
    kubernetes: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ParsedResponse {
    auth_token: AuthToken,
    orgs: Vec<Org>,
    roles: Vec<String>,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
}

#[allow(dead_code)]
impl HydrolixAuth {
    pub fn new(base_url: &str, username: &str, password: &str) -> Self {
        HydrolixAuth {
            base_url: base_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            http_client: reqwest::Client::new(),
            token: HydrolixToken {
                value: "".to_string(),
                org_list: vec![],
                expires_at: Instant::now(),
                hits: 0,
            },
        }
    }

    pub async fn get_token(mut self) -> Result<HydrolixToken, String> {
        let mut cache = TOKEN_CACHE.lock().await;

        // Check if token is cached and not expired
        if cache.expires_at >= Instant::now() {
            cache.hits += 1;
            return Ok(cache.clone());
        }

        // Fetch new token if not cached or expired
        let url = &format!("https://{}/config/v1/login", self.base_url);

        // Payload for the authentication request
        let payload = serde_json::json!({
            "username": self.username,
            "password": self.password,
        });

        // Send the authentication request
        let response = self
            .http_client
            .post(url.clone())
            .header(CONTENT_TYPE, "application/json")
            .timeout(Duration::from_secs(60))
            .json(&payload)
            .send()
            .await;

        let response2 = match response {
            Ok(v) => v, // Capture the response on success
            Err(e) => {
                return Err(format!(
                    "{}.{} Failed to authenticate: url={url} username={} password={} {e}",
                    file!(),
                    line!(),
                    self.username,
                    self.password
                ))
            }
        };

        // Check if the authentication was successful
        if !response2.status().is_success() {
            return Err(format!(
                "{}.{} Failed to authenticate: url={url} username={} password={}",
                file!(),
                line!(),
                self.username,
                self.password
            ));
        }

        let payload = match response2.text().await {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "{}.{} Failed to get json data {e}",
                    file!(),
                    line!()
                ))
            }
        };

        let parsed: ParsedResponse = match serde_json::from_str(&payload) {
            Ok(v) => v,
            Err(e) => return Err(format!("{}.{} Failed to parse data {e}", file!(), line!())),
        };

        for i in parsed.orgs {
            self.token.org_list.push(i.uuid.to_string());
        }
        self.token.value = parsed.auth_token.access_token.to_string();
        if let Some(v) = parsed.auth_token.expires_in {
            self.token.expires_at = Instant::now() + Duration::from_secs(v)
        }

        *cache = self.token;

        Ok(cache.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_token() {
        let base_url = "XXX";
        let username = "XXX";
        let password = "XXX";

        let auth = HydrolixAuth::new(&base_url, &username, &password);

        // Verify that the token is cached
        for i in 0..100 {
            match auth.clone().get_token().await {
                Ok(v) => assert!(v.hits == i),
                Err(e) => panic!("Failed to authenticate: {e}"),
            }
        }
    }
}
// Verify the token value and organization
