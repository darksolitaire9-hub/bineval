use crate::ports::ConfigPort;
use crate::core::suite::EvalSuite;
use crate::core::template::Template;
use std::fs;

pub struct JsonConfigAdapter;

impl ConfigPort for JsonConfigAdapter {
    fn load_templates(&self, repo_path: &str) -> anyhow::Result<Vec<Template>> {
        // Mock returning empty for now
        Ok(vec![])
    }

    fn load_suites(&self, repo_path: &str) -> anyhow::Result<Vec<EvalSuite>> {
        // Mock returning empty for now
        Ok(vec![])
    }
}
