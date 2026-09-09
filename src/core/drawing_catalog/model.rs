use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;

// The ultimate Output on success
pub struct SyncSummary {
    pub items_found: usize,
    pub items_inserted: usize,
}
pub enum SyncOutcome {
    Skipped { reason: String },
    Completed(SyncSummary),
}

// intermediate
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Catalog {
    pub file_path: PathBuf,
}

// input
pub struct SyncDirectoryCommand {
    pub target_dir: PathBuf,
}
