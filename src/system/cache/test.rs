use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

// ==========================================
// LAYER 1: The Domain Metadata Trait
// ==========================================
/// Implemented ONCE per database struct.
/// Contains ZERO I/O or file logic—only domain declarations.
pub trait Cacheable: Serialize + DeserializeOwned {
    /// The strongly-typed cache key (e.g., u64, String, or custom Uuid type)
    type Key: std::fmt::Display;

    /// The subfolder/namespace for this entity (e.g., "users")
    fn namespace() -> &'static str;

    /// How long a cached entry stays fresh
    fn ttl() -> Duration;
}

// --- Example Database Struct ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl Cacheable for User {
    type Key = u64;

    fn namespace() -> &'static str {
        "users"
    }

    fn ttl() -> Duration {
        Duration::from_secs(300) // 5 minutes
    }
}

// ==========================================
// LAYER 2: Envelope & Generic Storage Engine
// ==========================================

/// Internal wrapper that manages freshness metadata transparently.
#[derive(Serialize, Deserialize)]
struct CacheEnvelope<T> {
    cached_at: SystemTime,
    data: T,
}

/// The App-Wide Cache Engine.
/// Written ONCE for the entire application. Handles file paths, I/O, and TTL.
pub struct DiskCacheStore {
    base_dir: PathBuf,
}

impl DiskCacheStore {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Helper to resolve the path: base_dir / namespace / key.json
    fn resolve_path<T: Cacheable>(&self, key: &T::Key) -> PathBuf {
        self.base_dir.join(T::namespace()).join(format!("{}.json", key))
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

// ==========================================
// LAYER 3: Ergonomic Extension Trait (Optional)
// ==========================================
/// Gives you the `user.save_cache(&store)` dot-syntax ergonomics
/// without putting I/O logic inside the `User` struct itself.
pub trait CacheableExt: Cacheable {
    fn save_cache(&self, key: &Self::Key, store: &DiskCacheStore) -> Result<(), std::io::Error> {
        store.put::<Self>(key, self)
    }
}

// Blanket implementation: automatically applies to any struct implementing `Cacheable`
impl<T: Cacheable> CacheableExt for T {}

// ==========================================
// USAGE EXAMPLE
// ==========================================
fn main() {
    let store = DiskCacheStore::new("./my_app_cache");

    let user = User {
        id: 42,
        username: "ferris".to_string(),
        email: "ferris@rust-lang.org".to_string(),
    };

    // --- Option A: Engine syntax ---
    store.put(&user.id, &user).expect("Failed to save");
    
    // --- Option B: Extension Trait dot-syntax ---
    user.save_cache(&user.id, &store).expect("Failed to save via extension");

    // Fetching from cache (validates TTL under the hood)
    if let Some(cached_user) = store.get::<User>(&42) {
        println!("Cache Hit: {:?}", cached_user);
    } else {
        println!("Cache Miss or Expired!");
    }

    // Deleting from cache
    store.delete::<User>(&42).expect("Failed to delete");
}