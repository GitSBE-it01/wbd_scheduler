use crate::system::cache::FileCacheable;
use crate::system::cache::error::CacheError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;

pub struct FileCacheStore {
    root_dir: PathBuf,
}

impl FileCacheStore {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    /// Resolves full path: root + sub_dir + filename
    pub fn resolve_path<T: FileCacheable>(&self, key: Option<&str>) -> PathBuf {
        self.root_dir
            .join(T::cache_sub_dir())
            .join(T::cache_filename(key))
    }

    /// Check if cache file exists and is still within TTL
    pub async fn validate<T: FileCacheable>(
        &self,
        key: Option<&str>,
        custom_ttl: Option<Duration>,
    ) -> bool {
        let path = self.resolve_path::<T>(key);
        let Ok(meta) = fs::metadata(&path).await else {
            return false;
        };
        let Ok(modified) = meta.modified() else {
            return false;
        };
        let Ok(elapsed) = modified.elapsed() else {
            return false;
        };

        let ttl = custom_ttl.unwrap_or_else(T::default_ttl);
        elapsed < ttl
    }

    /// Save data as JSON
    pub async fn save<T: FileCacheable + Serialize>(
        &self,
        key: Option<&str>,
        data: &[T],
    ) -> Result<(), std::io::Error> {
        let path = self.resolve_path::<T>(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let json = serde_json::to_vec(data)?;
        fs::write(path, json).await
    }

    /// Delete cache file
    pub async fn delete<T: FileCacheable>(&self, key: Option<&str>) -> Result<(), std::io::Error> {
        let path = self.resolve_path::<T>(key);
        if path.exists() {
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct CacheWrapper<T> {
    data: T,
}

pub async fn read_data<T>(fullpath: impl AsRef<Path>) -> Result<Vec<T>, CacheError>
where
    T: DeserializeOwned,
{
    let json_content = fs::read_to_string(fullpath).await?;
    let wrapper: CacheWrapper<Vec<T>> = serde_json::from_str(&json_content)?;
    Ok(wrapper.data)
}
