use serde::{Deserialize, Serialize};
use super::credits::Credits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primitive {
    pub name: String,
    pub implementation_module: Option<String>,
    pub metadata_path: Option<String>,
    pub core_tag: String,
    pub imported: bool,
    #[serde(default)]
    pub credits: Credits,
}
