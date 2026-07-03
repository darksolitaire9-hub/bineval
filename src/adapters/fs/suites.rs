use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::policy::Policy;
use crate::core::ports::SuiteRepositoryPort;
use crate::core::suite::Suite;
use crate::core::BinevalError;

#[derive(Debug, Deserialize)]
struct FileSuite {
    #[serde(alias = "id")]
    suite_id: String,
    description: Option<String>,
    policies: Vec<Policy>,
}

pub struct FsSuiteRepositoryAdapter {
    pub root_dir: PathBuf,
}

impl FsSuiteRepositoryAdapter {
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    fn scan_dir(&self, dir: &Path, suites: &mut Vec<Suite>) -> Result<(), BinevalError> {
        if !dir.exists() || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir).map_err(|e| BinevalError::Unexpected {
            message: e.to_string(),
        })? {
            let entry = entry.map_err(|e| BinevalError::Unexpected {
                message: e.to_string(),
            })?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_dir(&path, suites)?;
            } else if let Some(ext) = path.extension() {
                if ext == "json" || ext == "yaml" || ext == "yml" {
                    let content =
                        fs::read_to_string(&path).map_err(|e| BinevalError::ParseError {
                            path: path.clone(),
                            source: Box::new(e),
                        })?;

                    let file_suite: FileSuite = if ext == "json" {
                        serde_json::from_str(&content).map_err(|e| BinevalError::ParseError {
                            path: path.clone(),
                            source: Box::new(e),
                        })?
                    } else {
                        serde_yaml::from_str(&content).map_err(|e| BinevalError::ParseError {
                            path: path.clone(),
                            source: Box::new(e),
                        })?
                    };

                    suites.push(Suite {
                        id: file_suite.suite_id,
                        description: file_suite.description,
                        policies: file_suite.policies,
                    });
                }
            }
        }
        Ok(())
    }
}

impl SuiteRepositoryPort for FsSuiteRepositoryAdapter {
    fn list_suites(&self) -> Result<Vec<Suite>, BinevalError> {
        let mut suites = Vec::new();
        let suites_dir = self.root_dir.join("suites");

        if suites_dir.exists() {
            self.scan_dir(&suites_dir, &mut suites)?;
        }

        Ok(suites)
    }

    fn get_suite(&self, id: &str) -> Result<Suite, BinevalError> {
        let suites = self.list_suites()?;
        suites
            .into_iter()
            .find(|s| s.id == id)
            .ok_or_else(|| BinevalError::SuiteNotFound {
                suite_id: id.to_string(),
            })
    }
}
