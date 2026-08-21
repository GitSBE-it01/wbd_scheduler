use crate::core::state::clock;
use crate::core::utilities::error::UtilError;
use rust_xlsxwriter::Workbook;
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use tokio::fs;
use tracing::info;

pub async fn save_as_json<T, P>(data: &T, dir_path: P, file_name: &str) -> Result<(), UtilError>
where
    T: Serialize + ?Sized,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir).await?;
    let file_path = dir.join(file_name);
    let created_at = clock::AppClock::new();
    let payload = json!({
        "expires_at": created_at.timestamp(),
        "data": data
    });
    let item_count = payload["data"].as_array().map(|arr| arr.len());
    let json_data = serde_json::to_string_pretty(&payload)?;
    fs::write(&file_path, json_data)
        .await
        .map_err(UtilError::write_file(&file_path))?;
    if let Some(count) = item_count {
        info!(
            count = count,
            path = ?file_path,
            "-->> Caching Done"
        );
    } else {
        info!(path = ?file_path, "-->> Caching Done");
    }
    Ok(())
}

pub async fn save_as_csv<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), UtilError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir)
        .await
        .map_err(UtilError::create_dir(dir))?;
    let file_path = dir.join(file_name);
    let mut writer = csv::Writer::from_writer(Vec::new());
    for record in data {
        writer.serialize(record)?; // Headers written automatically on first row
    }
    let csv_bytes = writer.into_inner().map_err(|e| e.into_error())?;
    fs::write(&file_path, csv_bytes)
        .await
        .map_err(UtilError::write_file(&file_path))?;
    info!(filepath = ?file_path, "-->> Successfully exported CSV to");
    Ok(())
}

pub async fn save_as_xlsx<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), UtilError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir)
        .await
        .map_err(UtilError::create_dir(dir))?;
    let file_path = dir.join(file_name);
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.serialize(&data)?;
    let excel_bytes = workbook.save_to_buffer()?;
    fs::write(&file_path, excel_bytes)
        .await
        .map_err(UtilError::write_file(&file_path))?;
    info!(filepath = ?file_path, "-->> Successfully exported XLSX to");
    Ok(())
}
