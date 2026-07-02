use super::credits::Credits;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primitive {
    pub name: String,
    pub implementation_status: Option<String>,
    pub metadata_path: Option<String>,
    pub requires_metadata: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub credits: Credits,
}
