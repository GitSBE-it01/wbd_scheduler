use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;
// use std::time::Instant;
// use tokio::fs;
// use tracing::{error, info};
use wbd_scheduler::{
    // core::config::AppConfig,
    core::error::AppResult,
    // core::state::AppState,
    core::utilities::read_cache::read_data,
};
#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct OldData {
    pub item: String,
    pub drawing_path: PathBuf,
}

#[tokio::test]
async fn cek_drawing_catalog() -> AppResult<()> {
    let fullpath = PathBuf::from(
        r"\\192.168.2.103\xampp\htdocs\wbd\_cache\pick_now\cache_data__drawing.cache",
    );
    let old: Vec<OldData> = read_data(&fullpath).await?;
    let fullpath = PathBuf::from(r".\cache\cache_data__drawing.json");
    let new: Vec<OldData> = read_data(&fullpath).await?;
    assert_eq!(
        old.len(),
        new.len(),
        "Record count mismatch! past: {}, New: {}",
        old.len(),
        new.len()
    );
    // assert_eq!(
    //     old, new,
    //     "Data discrepancy found between legacy DB data and new process output"
    // );
    Ok(())
}
