use crate::app_error::AppResult;
use crate::app_state::clock;
use std::path::Path;
use time::{Duration, OffsetDateTime};
use tokio::fs;

pub async fn is_cache_valid<P: AsRef<Path>>(file_path: P, max_age: Option<i64>) -> AppResult<bool> {
    let path = file_path.as_ref(); // Converts whatever path type is passed into &Path
    let ttl = Duration::hours(max_age.unwrap_or(12));
    if !fs::try_exists(path).await? {
        return Ok(false); // File does not exist
    }
    let clock = clock::AppClock::new();
    let metadata = fs::metadata(path).await?;
    if let Ok(modified) = metadata.modified() {
        let modified_time = OffsetDateTime::from(modified);
        let now = clock.now();

        if now - modified_time < ttl {
            return Ok(true); // Cache is still fresh
        }
    }
    Ok(false)
}
