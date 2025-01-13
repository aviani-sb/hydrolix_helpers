use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub project: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    pub uuid: String,
    pub url: String,
    pub created: String,
    pub modified: String,
}
