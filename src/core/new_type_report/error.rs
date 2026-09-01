use thiserror::Error;

#[derive(Debug, Error)]
pub enum NtrError {
    #[error("Database Insert failed: {0}")]
    DatabaseInsertFailed(String), // or sqlx::Error, etc.
    #[error("Failed Fetch Check Today Data: {0}")]
    ScalarFetchFail(#[from] sqlx::Error), // or sqlx::Error, etc.
    #[error("Failed Truncating data: {0}")]
    FailInsert(#[source] sqlx::Error), // or sqlx::Error, etc.
    #[error("Database failure: {0}")]
    Database(#[source] sqlx::Error),
}
