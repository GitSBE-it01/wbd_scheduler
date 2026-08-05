use crate::app_error::AppError;
use dotenvy::{dotenv, from_path};
use serde::Deserialize;
use std::path::Path;
use tracing::info;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub db_url_std: String,
    pub db_url_new: String,
}
impl AppConfig {
    pub fn new(path: &str) -> Result<Self, AppError> {
        let path = Path::new(path);
        let config = if path.exists() {
            from_path(path)?;
            envy::from_env::<AppConfig>()?
        } else {
            let _ = dotenv();
            envy::from_env::<AppConfig>()?
        };
        info!("AppConfig initialized: {:?}", config);
        Ok(config)
    }

    // pub fn from_env() -> Result<Self, AppError> {
    //     let _ = dotenv();
    //     let config = envy::from_env::<AppConfig>()?;
    //     Ok(config)
    // }
    // pub fn from_env_file(path: &Path) -> Result<Self, AppError> {
    //     from_path(path)?;
    //     let config = envy::from_env::<AppConfig>()?;
    //     Ok(config)
    // }
}
