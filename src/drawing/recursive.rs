use crate::app_error::AppResult;
use std::fs;
use std::io;
use std::path::Path;

pub fn get_filenames_from_dir(dir_path: &str) -> AppResult<Vec<String>> {
    let path = Path::new(dir_path);
    let mut filenames: Vec<String> = Vec::new();
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path does not exist: {}", path.display()),
        )
        .into());
    }
    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not a directory").into());
    }
    collect_filenames_recursive(path, path, &mut filenames)?;
    Ok(filenames)
}

pub fn collect_filenames_recursive(
    current_path: &Path,
    relative_base: &Path,
    filenames: &mut Vec<String>,
) -> AppResult<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // Get the directory name
            if let Some(dir_name_os) = entry_path.file_name() {
                if let Some(dir_name_str) = dir_name_os.to_str() {
                    let lowercased_dir_name = dir_name_str.to_lowercase();
                    if lowercased_dir_name.contains("superseded")
                        && lowercased_dir_name.contains("adr")
                    {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            collect_filenames_recursive(&entry_path, relative_base, filenames)?;
        } else if entry_path.is_file() {
            if let Some(relative_path) = entry_path.strip_prefix(relative_base).ok() {
                if let Some(name_str) = relative_path.to_str() {
                    // filenames.push(name_str.to_string());
                    let lowercased_name_str = name_str.to_lowercase();
                    if !lowercased_name_str.contains("superseded")
                        && !lowercased_name_str.contains("adr")
                    {
                        filenames.push(name_str.to_string());
                    }
                } else {
                    eprintln!(
                        "Warning: Skipping file with non-UTF-8 name: {:?}",
                        relative_path
                    );
                }
            }
        }
    }
    Ok(())
}
