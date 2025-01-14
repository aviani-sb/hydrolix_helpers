use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StorageSettings {
    pub bucket_name: String,
    pub bucket_path: String,
    pub region: String,
    pub endpoint: Option<String>,
    pub cloud: String,
    pub credential_id: Option<String>,
    pub account_name: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storage {
    pub name: String,
    pub org: String,
    pub description: String,
    pub uuid: String,
    pub url: String,
    pub created: String,
    pub modified: String,
    pub settings: StorageSettings,
    pub publish_task_id: Option<String>,
}
