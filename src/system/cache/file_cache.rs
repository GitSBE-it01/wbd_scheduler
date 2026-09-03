use crate::system::cache::Cacheable;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
struct CacheEnvelope<T> {
    cached_at: SystemTime,
    data: T,
}

pub struct Cache {
    base_dir: Option<PathBuf>,
}

impl Cache {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf().into(),
        }
    }

    /// Helper to resolve the path: base_dir / namespace / key.json
    fn resolve_path<T: Cacheable>(&self, key: &T::Key) -> PathBuf {
        self.base_dir
            .join(T::namespace())
            .join(format!("{}.json", key))
    }

    /// Generic Fetch: Reads file, deserializes envelope, checks TTL validation.
    pub fn get<T: Cacheable>(&self, key: &T::Key) -> Option<T> {
        let path = self.resolve_path::<T>(key);
        let content = fs::read_to_string(path).ok()?;

        let envelope: CacheEnvelope<T> = serde_json::from_str(&content).ok()?;

        // Validation check: ensure cache hasn't expired
        let elapsed = envelope.cached_at.elapsed().ok()?;
        if elapsed > T::ttl() {
            // Expired! Invalidate lazily.
            let _ = self.delete::<T>(key);
            return None;
        }

        Some(envelope.data)
    }

    /// Generic Save: Wraps value in metadata and writes to disk.
    pub fn put<T: Cacheable>(&self, key: &T::Key, data: &T) -> Result<(), std::io::Error> {
        let path = self.resolve_path::<T>(key);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let envelope = CacheEnvelope {
            cached_at: SystemTime::now(),
            data,
        };

        let json = serde_json::to_string_pretty(&envelope)?;
        fs::write(path, json)
    }

    /// Generic Delete
    pub fn delete<T: Cacheable>(&self, key: &T::Key) -> Result<(), std::io::Error> {
        let path = self.resolve_path::<T>(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

pub trait CacheableExt: Cacheable {
    fn save_cache(&self, key: &Self::Key, store: &DiskCacheStore) -> Result<(), std::io::Error> {
        store.put::<Self>(key, self)
    }
}

// Blanket implementation: automatically applies to any struct implementing `Cacheable`
impl<T: Cacheable> CacheableExt for T {}
