use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;
// input
pub struct ScanDirectoryCommand {
    pub target_dir: Vec<PathBuf>,
}
