use crate::app_error::AppResult;
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
    pub fn new(path: &str) -> AppResult<Self> {
        let path = Path::new(path);
        let config = if path.exists() {
            from_path(path)?;
            envy::from_env::<AppConfig>()?
        } else {
            let _ = dotenv();
            envy::from_env::<AppConfig>()?
        };
        info!("AppConfig initialized");
        Ok(config)
    }
}
