use crate::ports::AstPort;
use std::process::Command;
use std::path::Path;

pub struct PythonAstAdapter;

impl AstPort for PythonAstAdapter {
    fn parse_imports(&self, repo_path: &str) -> anyhow::Result<Vec<String>> {
        // We assume .bineval_ast_extractor.py exists in the current working dir of bineval
        let script_path = ".bineval_ast_extractor.py";
        
        let output = Command::new("uv")
            .arg("run")
            .arg("python")
            .arg(script_path)
            .arg(repo_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Python AST extractor failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let imports: Vec<String> = serde_json::from_str(&stdout)?;
        Ok(imports)
    }
}
