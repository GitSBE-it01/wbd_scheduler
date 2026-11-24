use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory Not Found")]
    DirNotFound,
    #[error("Path is not a Directory")]
    InvalidInput,
}
