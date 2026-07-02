use crate::core::errors::ConfigError;
use crate::core::suite::EvalSuite;
use crate::core::template::Template;
use crate::ports::ConfigPort;

pub struct JsonConfigAdapter;

impl ConfigPort for JsonConfigAdapter {
    fn load_templates(&self, _repo_path: &str) -> Result<Vec<Template>, ConfigError> {
        // Mock returning empty for now
        Ok(vec![])
    }

    fn load_suites(&self, _repo_path: &str) -> Result<Vec<EvalSuite>, ConfigError> {
        // Mock returning empty for now
        Ok(vec![])
    }
}
