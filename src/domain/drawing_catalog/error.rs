use std::path::PathBuf;
use thiserror::Error;

use crate::core;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Read Directory failed: {0}")]
    ReadDirectoryFailed(std::io::Error),
    #[error("Database Insert failed: {0}")]
    DatabaseInsertFailed(String), // or sqlx::Error, etc.
    #[error("Failed Fetch Check Today Data: {0}")]
    ScalarFetchFail(#[from] sqlx::Error), // or sqlx::Error, etc.
    #[error("Failed Truncating data: {0}")]
    TruncateFailed(#[source] sqlx::Error), // or sqlx::Error, etc.
    #[error("Failed Truncating data: {0}")]
    FailInsert(#[source] sqlx::Error), // or sqlx::Error, etc.
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory is not found: {0}")]
    DirNotFound(PathBuf),
    #[error("Path is not directory: {0}")]
    InvalidInput(PathBuf),
    #[error("Path is not directory: {0}")]
    Database(#[from] core::error::AppError),
    #[error("Path is not directory: {0}")]
    UtilError(#[from] core::utilities::error::UtilError),
}
