use crate::system;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(#[from] system::config::error::ConfigError),
    #[error("State error: {0}")]
    State(#[from] system::state::error::StateError),
    #[error("Cache Error: {0}")]
    CacheError(#[from] system::cache::error::CacheError),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] system::db::error::DbError),
    #[error("IO Error: {0}")]
    IoError(#[from] system::io::error::IoError),
    // #[error("Utilities error: {0}")]
    // NewTypeError(#[from] core::new_type_report::error::NtrError),
}
