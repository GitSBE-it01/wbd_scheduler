use crate::core::drawing_catalog::scan_dir::collect_filenames_recursive;
use crate::core::drawing_catalog::{error::SyncError, model::Catalog};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub async fn parse_directory_collection(rec_dir: &[PathBuf]) -> Result<Vec<Catalog>, SyncError> {
    let mut catalog: Vec<Catalog> = Vec::new();
    for dir in rec_dir {
        let path = Path::new(dir);
        if !path.exists() {
            return Err(SyncError::DirNotFound(path.to_path_buf()));
        }
        if !path.is_dir() {
            return Err(SyncError::InvalidInput(path.to_path_buf()));
        }
        collect_filenames_recursive(path, path, &mut catalog)?;
    }
    Ok(catalog)
}
