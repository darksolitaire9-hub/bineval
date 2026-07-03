pub mod policy;
pub mod ports;
pub mod primitive;
pub mod result;
pub mod suite;
pub mod usecases;

#[derive(thiserror::Error, Debug)]
pub enum BinevalError {
    #[error("config file not found: {path}")]
    ConfigNotFound { path: std::path::PathBuf },

    #[error("failed to parse {path}: {source}")]
    ParseError {
        path: std::path::PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("suite not found: {suite_id}")]
    SuiteNotFound { suite_id: String },

    #[error("policy '{policy_id}' failed for primitive '{primitive_id}': {message}")]
    PolicyFailed {
        suite_id: String,
        policy_id: String,
        primitive_id: String,
        message: String,
    },

    #[error("unexpected error: {message}")]
    Unexpected { message: String },
}
