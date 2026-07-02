use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credits {
    #[serde(default)]
    pub humans: Vec<HumanCredit>,
    #[serde(default)]
    pub organizations: Vec<OrgCredit>,
    #[serde(default)]
    pub models: Vec<ModelCredit>,
    #[serde(default)]
    pub tools: Vec<ToolCredit>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanCredit {
    pub name: String,
    pub email_or_handle: Option<String>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCredit {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCredit {
    pub model_name: String,
    pub provider: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCredit {
    pub name: String,
    pub version: Option<String>,
    pub role: String,
}
