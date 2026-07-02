use super::credits::Credits;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub category: String,
    pub inputs: TemplateInputs,
    pub checks: Vec<PolicyRef>,
    pub execution: ExecutionSpec,
    #[serde(default)]
    pub credits: Credits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateInputs {
    #[serde(rename = "dataset")]
    Dataset { path: String },
    #[serde(rename = "generator")]
    Generator { generator_ref: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRef {
    pub policy_id: String,
    pub severity: String, // e.g. "error", "warn"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSpec {
    pub target_model: Option<String>,
    pub target_endpoint: Option<String>,
    pub max_cases: Option<u32>,
    pub timeout_ms: Option<u32>,
}
