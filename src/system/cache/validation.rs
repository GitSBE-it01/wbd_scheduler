use crate::system::{cache::error::CacheError, state::clock};
use std::path::Path;
use time::OffsetDateTime;
use tokio::fs;

pub async fn is_cache_valid<P: AsRef<Path>>(
    file_path: P,
    max_age: Option<i64>,
) -> Result<bool, CacheError> {
    let path = file_path.as_ref();
    if !fs::try_exists(path).await? {
        return Ok(false);
    }
    let max_age_secs = max_age.unwrap_or(43200); // Default 12 hours in seconds
    let clock = clock::AppClock::new();
    let metadata = fs::metadata(path).await?;
    if let Ok(modified) = metadata.modified() {
        let modified_time = OffsetDateTime::from(modified);
        let now = clock.now();
        let elapsed_secs = (now - modified_time).whole_seconds();
        if elapsed_secs >= 0 && elapsed_secs < max_age_secs {
            return Ok(true);
        }
    }
    Ok(false)
}
