use crate::core::utilities::error::UtilError;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::fs;

#[derive(Deserialize)]
struct CacheWrapper<T> {
    data: T,
}

pub async fn read_data<T>(fullpath: impl AsRef<Path>) -> Result<Vec<T>, UtilError>
where
    T: DeserializeOwned,
{
    let json_content = fs::read_to_string(fullpath).await?;
    let wrapper: CacheWrapper<Vec<T>> = serde_json::from_str(&json_content)?;
    Ok(wrapper.data)
}
