use crate::system::io::error::IoError;
use csv::{ReaderBuilder, StringRecord};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use tokio::fs;

/// Reads a CSV file and deserializes it into any struct type T
pub fn load_csv_data<T, P>(path: P, custom_headers: &[&str]) -> Result<Vec<T>, IoError>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    // 1. Transcode ISO-8859-1 stream into UTF-8 on the fly
    let utf8_stream = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut reader = ReaderBuilder::new().from_reader(utf8_stream);

    // 2. Parse original headers to consume row 1 and validate column count
    let actual_col_count = reader.headers()?.len();
    if actual_col_count != custom_headers.len() {
        return Err(IoError::CsvHeaderMismatch);
    }

    // 3. Inject custom headers for position-based Serde field mapping
    reader.set_headers(StringRecord::from(custom_headers));

    // 4. Deserialize rows using injected header names
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub async fn save_as_csv<T, P>(data: &[T], dir_path: P, file_name: &str) -> Result<(), IoError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    if data.len() == 0 {
        return Err(IoError::EmptyData);
    }
    let dir = dir_path.as_ref();
    fs::create_dir_all(dir)
        .await
        .map_err(IoError::create_dir(dir))?;
    let file_path = dir.join(file_name);
    let mut writer = csv::Writer::from_writer(Vec::new());
    for record in data {
        writer.serialize(record)?; // Headers written automatically on first row
    }
    let csv_bytes = writer.into_inner().map_err(|e| e.into_error())?;
    fs::write(&file_path, csv_bytes)
        .await
        .map_err(IoError::write_file(&file_path))?;
    Ok(())
}
