use crate::core::primitive::Primitive;
use crate::core::suite::EvalSuite;
use crate::core::template::Template;
use crate::core::credits::Credits;
use crate::core::errors::{ConfigError, AstError};
use serde_json::Value;

pub trait RepoPort {
    fn read_file(&self, path: &str) -> anyhow::Result<String>;
    fn list_files(&self, root: &str, pattern: &str) -> anyhow::Result<Vec<String>>;
}

pub trait AstPort {
    // Note: Rust >= 1.75 allows async fn in traits
    fn parse_imports(&self, repo_path: &str) -> impl std::future::Future<Output = Result<Vec<String>, AstError>> + Send;
}

pub trait ConfigPort {
    fn load_templates(&self, repo_path: &str) -> Result<Vec<Template>, ConfigError>;
    fn load_suites(&self, repo_path: &str) -> Result<Vec<EvalSuite>, ConfigError>;
}

pub trait CreditsPort {
    fn load_project_credits(&self, repo_path: &str) -> anyhow::Result<Credits>;
    fn load_entity_credits(&self, entity_name: &str) -> anyhow::Result<Credits>;
}

pub trait OutputPort {
    fn write_json(&self, path: &str, data: &Value) -> anyhow::Result<()>;
    fn print_report(&self, report: &str) -> anyhow::Result<()>;
}
