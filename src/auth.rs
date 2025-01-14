use once_cell::sync::Lazy;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Org {
    pub uuid: String,
    pub name: String,
    pub cloud: String,
    #[serde(default)]
    pub kubernetes: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HydrolixToken {
    pub value: String,
    pub org_list: Vec<Org>,
    pub expires_at: Instant,
    pub hits: usize,
    pub base_url: String,
}

impl Default for HydrolixToken {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl HydrolixToken {
    pub fn new() -> HydrolixToken {
        HydrolixToken {
            value: "".to_string(),
            org_list: vec![],
            expires_at: Instant::now(),
            hits: 0,
            base_url: "".to_string(),
        }
    }
    pub fn first_org(self) -> String {
        match self.org_list.first() {
            Some(v) => v.name.to_string(),
            None => "".to_string(),
        }
    }
}

#[allow(dead_code)]
static TOKEN_CACHE: Lazy<Mutex<HydrolixToken>> = Lazy::new(|| Mutex::new(HydrolixToken::new()));

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HydrolixAuth {
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
    pub async fn new(base_url: &str, username: &str, password: &str) -> Self {
        let token: HydrolixToken = HydrolixToken {
            base_url: base_url.to_string(),
            value: "".to_string(),
            org_list: vec![],
            expires_at: Instant::now(),
            hits: 0,
        };
        let mut cache = TOKEN_CACHE.lock().await;
        *cache = token.clone();

        HydrolixAuth {
            base_url: base_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            http_client: reqwest::Client::new(),
            token: token.clone(),
        }
    }

    pub fn get_base_url(self) -> String {
        self.token.base_url.to_string()
    }

    pub async fn get_token(mut self) -> Result<HydrolixToken, String> {
        {
            let mut cache = TOKEN_CACHE.lock().await;

            // Check if token is cached and not expired
            if cache.expires_at >= Instant::now() {
                cache.hits += 1;
                return Ok(cache.clone());
            }
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

        for o in &parsed.orgs {
            self.token.org_list.push(o.clone());
        }
        self.token.value = parsed.auth_token.access_token.to_string();
        if let Some(v) = parsed.auth_token.expires_in {
            self.token.expires_at = Instant::now() + Duration::from_secs(v)
        }

        self.token.base_url = self.base_url.to_string();

        {
            let mut cache = TOKEN_CACHE.lock().await;

            *cache = self.token;

            Ok(cache.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::HydrolixAuth;
    use crate::auth::HydrolixToken;
    use crate::hydrolix;
    use std::fs;

    #[tokio::test]
    async fn test_token() {
        let x = HydrolixToken::new();
        assert!(x.hits == 0);
    }

    #[tokio::test]
    async fn test_get_token() {
        let file_path = "/tmp/fleet.secrets.toml";

        // Read the file into a string
        let content = match fs::read_to_string(file_path) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Failed to read file: {e}"),
        };

        // Parse the TOML content into the Config struct
        let config: hydrolix::secrets::Config = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse config: {e}"),
        };

        for m in &config.machines {
            assert!(!m.base_url.is_empty());

            let auth = HydrolixAuth::new(&m.base_url, &m.username, &m.password).await;

            match auth.clone().get_token().await {
                Ok(_) => (),
                Err(e) => panic!("Failed to authenticate: {e}"),
            }

            // Verify that the token is cached
            for i in 1..100 {
                match auth.clone().get_token().await {
                    Ok(v) => {
                        assert!(v.hits == i);
                        assert!(!v.first_org().is_empty());
                    }
                    Err(e) => panic!("Failed to authenticate: {e}"),
                }
            }
        }
    }
}
// Verify the token value and organization
