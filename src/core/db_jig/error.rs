use crate::system;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JigError {
    #[error("General Jig failed: {0}")]
    JigError(String), // or sqlx::Error, etc.
    #[error("Error parsing:  {0}")]
    SyncError(#[from] crate::core::drawing_catalog::error::SyncError), // or sqlx::Error, etc.
}
