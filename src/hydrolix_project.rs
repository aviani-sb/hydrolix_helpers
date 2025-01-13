use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hydrolix_function;
use crate::hydrolix_table;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub org: String,
    pub description: Option<String>, // Nullable field
    pub uuid: String,
    pub url: String,
    pub created: String, // Use `chrono::DateTime` for date parsing if needed
    pub modified: String,
    pub settings: Settings,
    pub tables: Option<Vec<hydrolix_table::Table>>,
    pub functions: Option<Vec<hydrolix_function::Function>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub default_query_options: Value, // Generic JSON for flexibility
    pub blob: Option<Value>,          // Nullable field for JSON blob
    pub rate_limit: Option<Value>,    // Nullable field for rate limits
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectList {
    pub projects: Vec<Project>,
}
