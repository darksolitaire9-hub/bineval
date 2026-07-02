use crate::ports::RepoPort;
use std::fs;
use std::path::Path;

pub struct FsRepoAdapter;

impl RepoPort for FsRepoAdapter {
    fn read_file(&self, path: &str) -> anyhow::Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    fn list_files(&self, root: &str, pattern: &str) -> anyhow::Result<Vec<String>> {
        // Basic implementation just listing all files, could use glob or walkdir in production
        let mut result = Vec::new();
        let root_path = Path::new(root);
        if root_path.exists() && root_path.is_dir() {
            for entry in walkdir::WalkDir::new(root_path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path_str = entry.path().to_string_lossy().to_string();
                    if path_str.contains(pattern) {
                        result.push(path_str);
                    }
                }
            }
        }
        Ok(result)
    }
}
