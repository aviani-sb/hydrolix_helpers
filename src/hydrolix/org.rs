use serde::{Deserialize, Serialize};

use crate::hydrolix::project;
use crate::hydrolix::storage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Org {
    pub uuid: String,
    pub name: String,
    pub cloud: String,
    #[serde(default)]
    pub kubernetes: bool,
    pub projects: Option<Vec<project::Project>>,
    pub storages: Option<Vec<storage::Storage>>,
}
