pub mod error;
use crate::system::db::DatabaseCacheable;
use crate::system::state::AppState;
use error::CacheError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs;
use std::path::{Path, PathBuf};
use time::{Duration, OffsetDateTime};
use tracing::info;
pub trait Cacheable: Serialize + DeserializeOwned {
    type Key: std::fmt::Display;
    fn namespace() -> &'static str;
    fn ttl() -> i64;
}

#[derive(Serialize, Deserialize)]
pub struct CacheResult<T> {
    expires_at: String,
    pub data: Vec<T>,
}

pub struct CacheEngine {
    base_dir: PathBuf,
}

impl CacheEngine {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Helper to resolve the path: base_dir / namespace / key.json
    fn resolve_path<T: Cacheable>(&self, key: &T::Key) -> PathBuf {
        self.base_dir
            .join(T::namespace())
            .join(format!("{}.json", key))
    }

    pub async fn db_handler<T>(
        &self,
        state: &AppState,
        key: T::Key,
    ) -> Result<CacheResult<T>, CacheError>
    where
        T: DatabaseCacheable + Clone,
    {
        if let Some(cached_data) = self.get::<T>(state, &key) {
            return Ok(cached_data);
        }
        let fresh_data: Vec<T> = T::fetch_db(state).await?;
        let result = self.put::<T>(state, &key, fresh_data)?;
        Ok(result)
    }

    // pub fn get<T: Cacheable>(&self, state: &AppState, key: &T::Key) -> Option<Vec<T>>
    pub fn get<T: Cacheable>(&self, state: &AppState, key: &T::Key) -> Option<CacheResult<T>> {
        let path = self.resolve_path::<T>(key);
        let content = fs::read_to_string(&path).ok()?;
        let now = state.clock.now();
        let envelope: CacheResult<T> = match serde_json::from_str(&content) {
            Ok(env) => env,
            Err(_) => {
                let _ = fs::remove_file(&path);
                return None;
            }
        };
        let created_at = match state.clock.parse_local_str(&envelope.expires_at) {
            Ok(dt) => dt,
            Err(_) => {
                let _ = self.delete::<T>(key);
                return None;
            }
        };
        let elapsed = now.unix_timestamp() - created_at.unix_timestamp();
        if elapsed.is_negative() || elapsed > T::ttl() {
            let _ = self.delete::<T>(key);
            return None;
        }
        Some(envelope)
    }

    pub fn put<T: Cacheable>(
        &self,
        state: &AppState,
        key: &T::Key,
        data: Vec<T>,
    ) -> Result<CacheResult<T>, CacheError> {
        let path = self.resolve_path::<T>(key);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let envelope = CacheResult {
            expires_at: state.clock.timestamp(),
            data,
        };
        let json = serde_json::to_string_pretty(&envelope)?;
        let cnt = json.len();
        fs::write(path, json)?;
        info!(data=?cnt, key= T::namespace(), "New Cache Done");
        Ok(envelope)
    }

    /// Generic Delete
    pub fn delete<T: Cacheable>(&self, key: &T::Key) -> Result<(), CacheError> {
        let path = self.resolve_path::<T>(key);
        if path.exists() {
            info!(ChacheKey = %key, "Cache Invalid");
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
