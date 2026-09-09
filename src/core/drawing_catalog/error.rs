use crate::system;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Read Directory failed: {0}")]
    ReadDirectoryFailed(std::io::Error),
    #[error("Database Insert failed: {0}")]
    DatabaseInsertFailed(String), // or sqlx::Error, etc.
    #[error("Path is not directory: {0}")]
    Database(#[from] system::db::error::DbError),
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory is not found: {0}")]
    DirNotFound(PathBuf),
    #[error("Path is not directory: {0}")]
    InvalidInput(PathBuf),
    #[error("Path is not directory: {0}")]
    CacheError(#[from] system::cache::error::CacheError),
}
