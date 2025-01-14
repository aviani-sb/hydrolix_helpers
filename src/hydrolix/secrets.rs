use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Machine {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub machines: Vec<Machine>,
}
