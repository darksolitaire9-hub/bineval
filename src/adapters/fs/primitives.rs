use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::ports::PrimitiveRepositoryPort;
use crate::core::primitive::Primitive;
use crate::core::BinevalError;

pub struct FsPrimitiveRepositoryAdapter {
    pub root_dir: PathBuf,
}

impl FsPrimitiveRepositoryAdapter {
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    fn scan_dir(&self, dir: &Path, primitives: &mut Vec<Primitive>) -> Result<(), BinevalError> {
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
                self.scan_dir(&path, primitives)?;
            } else if let Some(ext) = path.extension() {
                if ext == "json" || ext == "yaml" || ext == "yml" {
                    let content =
                        fs::read_to_string(&path).map_err(|e| BinevalError::ParseError {
                            path: path.clone(),
                            source: Box::new(e),
                        })?;

                    let data: Value = if ext == "json" {
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

                    let id = if let Some(id_val) = data.get("id") {
                        if let Some(id_str) = id_val.as_str() {
                            id_str.to_string()
                        } else {
                            path.file_stem().unwrap().to_string_lossy().into_owned()
                        }
                    } else {
                        path.file_stem().unwrap().to_string_lossy().into_owned()
                    };

                    primitives.push(Primitive { id, data });
                }
            }
        }
        Ok(())
    }
}

impl PrimitiveRepositoryPort for FsPrimitiveRepositoryAdapter {
    fn list_primitives(&self) -> Result<Vec<Primitive>, BinevalError> {
        let mut primitives = Vec::new();
        
        // Scan the explicit directory provided, no domain-specific "potentials" logic
        self.scan_dir(&self.root_dir, &mut primitives)?;

        Ok(primitives)
    }
}
