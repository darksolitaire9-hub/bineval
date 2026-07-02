use crate::ports::AstPort;
use crate::core::errors::AstError;
use tokio::process::Command;

pub struct PythonAstAdapter;

impl AstPort for PythonAstAdapter {
    async fn parse_imports(&self, repo_path: &str) -> Result<Vec<String>, AstError> {
        let script_path = ".bineval_ast_extractor.py";
        
        let output = Command::new("uv")
            .arg("run")
            .arg("python")
            .arg(script_path)
            .arg(repo_path)
            .output()
            .await
            .map_err(AstError::SpawnFailed)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(AstError::HelperFailed(output.status.code().unwrap_or(-1), stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let imports: Vec<String> = serde_json::from_str(&stdout).map_err(AstError::ParseFailed)?;
        Ok(imports)
    }
}
