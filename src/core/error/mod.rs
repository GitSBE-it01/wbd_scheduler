use crate::core;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(#[from] core::config::error::ConfigError),
    #[error("State error: {0}")]
    State(#[from] core::state::error::StateError),
    #[error("Utilities error: {0}")]
    Utilities(#[from] core::utilities::error::UtilError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    // -- IO and Parsing Errors
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Cache data not available or invalid at path: {0:?}")]
    CacheMiss(PathBuf),
    #[error("Serialize failed: {0}")]
    Serialize(#[from] serde_json::Error),
    // #[error("Integer parsing failed: {0}")]
    // ParseInt(#[from] std::num::ParseIntError),
    // #[error("CSV error: {0}")]
    // Csv(#[from] csv::Error),
    // #[error("Excel error: {0}")]
    // Xlsx(#[from] rust_xlsxwriter::XlsxError),
    // -- Execution and Resource Errors
    // #[error("Resource not found: {0}")]
    // NotFound(String),
    // #[error("Command execution failed: {0}")]
    // ExecutionFailed(String),

    // #[error("Failed to create directory '{path}': {source}")]
    // CreateDir {
    //     path: PathBuf,
    //     source: std::io::Error,
    // },
    // #[error("Failed to write file '{path}': {source}")]
    // WriteFile {
    //     path: PathBuf,
    //     source: std::io::Error,
    // },
}

pub type AppResult<T> = std::result::Result<T, AppError>;
