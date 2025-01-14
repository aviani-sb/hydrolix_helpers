use crate::hydrolix::org;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cluster {
    pub base_url: String,
    pub orgs: Option<Vec<org::Org>>,
}
