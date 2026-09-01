use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Failed to build database connection pool: {0}")]
    DatabaseConnection(#[source] sqlx::Error),
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheck(#[source] sqlx::Error),
    #[error("Time formatting error: {0}")]
    TimeFormat(#[from] time::error::Format),
}
