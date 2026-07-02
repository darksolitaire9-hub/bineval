use serde::{Deserialize, Serialize};
use super::credits::Credits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSuite {
    pub name: String,
    pub category: String,
    pub templates: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub credits: Credits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyComponent {
    pub name: String,
    pub module_path: Option<String>,
    pub imported: bool,
}
