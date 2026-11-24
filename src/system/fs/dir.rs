use super::error::FsError;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct RootDir(PathBuf);

impl RootDir {
    // If it doesn't need to be async, you can remove `async` entirely:
    pub fn parse(dir_path: &str) -> Result<Self, FsError> {
        let path = PathBuf::from(dir_path);

        if !path.exists() {
            return Err(FsError::DirNotFound);
        }
        if !path.is_dir() {
            return Err(FsError::InvalidInput);
        }

        Ok(Self(path))
    }

    // Fixed: added `fn`, included `&self`, and removed the semicolon
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

pub struct FilesCollection(Vec<PathBuf>);
pub fn files_collection(
    current_path: RootDir,
    _relative_base: &Path,
) -> Result<Vec<PathBuf>, FsError> {
    let mut records: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(current_path.as_path())? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // 2. Flattened directory check using to_string_lossy
            let dir_name = entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            if dir_name.contains("superseded") && dir_name.contains("adr") {
                continue;
            }
            collect_filenames_recursive(&entry_path, _relative_base, catalog)?;
        } else if entry_path.is_file() {
            // 3. Calculated relative path safely
            catalog.push(Catalog {
                file_path: entry_path,
            });
        }
    }
    Ok()
}
