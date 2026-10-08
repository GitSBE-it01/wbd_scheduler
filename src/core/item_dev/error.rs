use crate::system;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemDevError {
    #[error("General error: {0}")]
    General(String),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] system::db::error::DbError),
    #[error("Caching Error: {0}")]
    CacheError(#[from] system::cache::error::CacheError),
}
