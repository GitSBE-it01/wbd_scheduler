pub mod error;
pub mod file_cache;
pub mod validation;

use std::path::Path;
use std::time::Duration;

pub trait FileCacheable {
    fn cache_filename(key: Option<&str>) -> String;
    fn default_ttl() -> Duration;
    fn cache_sub_dir() -> &'static Path;
}
