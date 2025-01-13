use serde::{Deserialize, Serialize};

use crate::hydrolix_project;
use crate::hydrolix_storage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Org {
    pub uuid: String,
    pub name: String,
    pub cloud: String,
    #[serde(default)]
    pub kubernetes: bool,
    pub projects: Option<Vec<hydrolix_project::Project>>,
    pub storages: Option<Vec<hydrolix_storage::Storage>>,
}
