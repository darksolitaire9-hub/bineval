use crate::core::policy::Policy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suite {
    pub id: String,
    pub description: Option<String>,
    pub policies: Vec<Policy>,
}
