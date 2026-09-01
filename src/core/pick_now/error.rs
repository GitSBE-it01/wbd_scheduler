use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PnError {
    #[error("General error: {0}")]
    General(String),
}
