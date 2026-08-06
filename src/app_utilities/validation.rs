use crate::app_error::AppResult;
use std::path::Path;
use time::{Duration, OffsetDateTime};
use tokio::fs;

pub async fn is_cache_valid<P: AsRef<Path>>(file_path: P, max_age: Duration) -> AppResult<bool> {
    let path = file_path.as_ref(); // Converts whatever path type is passed into &Path
    if !fs::try_exists(path).await? {
        return Ok(false); // File does not exist
    }
    let metadata = fs::metadata(path).await?;
    if let Ok(modified) = metadata.modified() {
        let modified_time = OffsetDateTime::from(modified);
        let now = OffsetDateTime::now_utc();

        if now - modified_time < max_age {
            return Ok(true); // Cache is still fresh
        }
    }
    Ok(false)
}
