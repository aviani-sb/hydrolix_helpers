use crate::hydrolix_org;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cluster {
    pub base_url: String,
    pub orgs: Option<Vec<hydrolix_org::Org>>,
}
