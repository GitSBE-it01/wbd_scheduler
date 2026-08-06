use crate::app_error::{AppError, AppResult};
use rust_xlsxwriter::Workbook;
use serde::Serialize;
use std::path::Path;
use tokio::fs;

pub async fn save_as_json<T, P>(data: &T, dir_path: P, file_name: &str) -> AppResult<()>
where
    T: Serialize + ?Sized,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir).await?;
    let file_path = dir.join(file_name);
    let json_data = serde_json::to_string_pretty(data)?;
    fs::write(&file_path, json_data).await?;
    tracing::info!("Successfully exported JSON to {:?}", file_path);
    Ok(())
}

pub async fn save_as_csv<T, P>(data: &[T], dir_path: P, file_name: &str) -> AppResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir).await?;
    let file_path = dir.join(file_name);
    let mut writer = csv::Writer::from_writer(Vec::new());
    for record in data {
        writer.serialize(record)?; // Headers written automatically on first row
    }
    let csv_bytes = writer.into_inner().map_err(|e| e.into_error())?;
    fs::write(&file_path, csv_bytes)
        .await
        .map_err(|err| AppError::WriteFile {
            path: file_path.clone(),
            source: err,
        })?;
    tracing::info!("Successfully exported CSV to {:?}", file_path);
    Ok(())
}

pub async fn save_as_xlsx<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), AppError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir)
        .await
        .map_err(|err| AppError::CreateDir {
            path: dir.to_path_buf(),
            source: err,
        })?;
    let file_path = dir.join(file_name);
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.serialize(&data)?;
    let excel_bytes = workbook.save_to_buffer()?;
    fs::write(&file_path, excel_bytes)
        .await
        .map_err(|err| AppError::WriteFile {
            path: file_path.clone(),
            source: err,
        })?;
    tracing::info!("Successfully exported XLSX to {:?}", file_path);
    Ok(())
}
