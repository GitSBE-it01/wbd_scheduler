use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // --- 1. Foreign Errors (Mapped automatically via #[from]) ---
    #[error("Dotenv loading failed: {0}")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Configuration error: {0}")]
    Envy(#[from] envy::Error),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Failed to build database connection pool: {0}")]
    DatabaseConnection(#[source] sqlx::Error),

    #[error("Database health check failed: {0}")]
    DatabaseHealthCheck(#[source] sqlx::Error),

    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Integer parsing failed: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
}
