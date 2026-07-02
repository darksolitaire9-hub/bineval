use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Suite '{0}' not found in registry")]
    SuiteNotFound(String),
    #[error("Template '{0}' missing required checks")]
    InvalidTemplate(String),
    #[error("Policy id '{0}' is not registered")]
    UnknownPolicy(String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Helper script failed with exit code {0}: {1}")]
    HelperFailed(i32, String),
    #[error("Failed to spawn helper script: {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("Failed to parse JSON output from helper: {0}")]
    ParseFailed(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Endpoint {0} unavailable: {1}")]
    EndpointUnavailable(String, String),
    #[error("Timeout for case {0}")]
    Timeout(String),
}

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("AST Partial Failure: {0}")]
    AstPartial(#[from] AstError),
}
