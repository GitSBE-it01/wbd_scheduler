pub mod error;

pub fn recursive_collect_filenames(
    current_path: &Path,
    _relative_base: &Path,
    catalog: &mut Vec<T>,
) -> Result<(), FsError> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
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
            catalog.push(T {
                file_path: entry_path,
            });
        }
    }
    Ok(())
}
