use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub machines: Vec<Machine>,
    pub git_snapshots: GitSnapshots,
}

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GitSnapshots {
    pub token: String,
}
