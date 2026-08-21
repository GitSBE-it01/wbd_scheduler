use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // -- Environment and Configuration Errors
    #[error("Dotenv loading failed: {0}")]
    Dotenv(#[from] dotenvy::Error),
    #[error("Configuration error: {0}")]
    Envy(#[from] envy::Error),
}
