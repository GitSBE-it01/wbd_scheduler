use crate::core::drawing_catalog::{error::SyncError, model::Catalog};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub async fn parse_directory(dir_path: &PathBuf) -> Result<Vec<Catalog>, SyncError> {
    let path = Path::new(dir_path);
    let mut catalog: Vec<Catalog> = Vec::new();
    if !path.exists() {
        return Err(SyncError::DirNotFound(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(SyncError::InvalidInput(path.to_path_buf()));
    }
    collect_filenames_recursive(path, path, &mut catalog)?;
    Ok(catalog)
}

pub fn collect_filenames_recursive(
    current_path: &Path,
    relative_base: &Path,
    catalog: &mut Vec<Catalog>,
) -> Result<(), SyncError> {
    for entry in fs::read_dir(current_path)? {
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
            collect_filenames_recursive(&entry_path, relative_base, catalog)?;
        } else if entry_path.is_file() {
            // 3. Calculated relative path safely
            catalog.push(Catalog {
                file_path: entry_path,
            });
        }
    }
    Ok(())
}
