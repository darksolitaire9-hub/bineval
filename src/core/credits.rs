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
    #[serde(default)]
    pub upstream_influences: Vec<UpstreamInfluence>,
    #[serde(default)]
    pub original_contributions: Vec<OriginalContribution>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamInfluence {
    pub title: String,
    pub arxiv: Option<String>,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalContribution {
    pub feature: String,
    pub purpose: String,
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
    pub provider: Option<String>,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCredit {
    pub name: String,
    pub url: Option<String>,
    pub purpose: String,
}
