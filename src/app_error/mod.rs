use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // -- Environment and Configuration Errors
    #[error("Dotenv loading failed: {0}")]
    Dotenv(#[from] dotenvy::Error),
    #[error("Configuration error: {0}")]
    Envy(#[from] envy::Error),

    // -- Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Failed to build database connection pool: {0}")]
    DatabaseConnection(#[source] sqlx::Error),
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheck(#[source] sqlx::Error),

    // -- IO and Parsing Errors
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Integer parsing failed: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Serialize failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Time formatting error: {0}")]
    TimeFormat(#[from] time::error::Format),
    #[error("Excel error: {0}")]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),

    // -- Execution and Resource Errors
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Cache data not available or invalid at path: {0:?}")]
    CacheMiss(PathBuf),
    #[error("Failed to create directory '{path}': {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to write file '{path}': {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub type AppResult<T> = core::result::Result<T, AppError>;
