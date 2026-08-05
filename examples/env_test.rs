use dotenvy::from_path;
use serde::Deserialize;
use std::path::Path;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
// use std::sync::Arc;
//, layer::SubscriberExt, util::SubscriberInitExt
#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub db_url_std: String,
    pub db_url_new: String,
}
impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<AppConfig>()
    }
    pub fn from_env_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        from_path(path)?;
        Ok(envy::from_env::<AppConfig>()?)
    }
}

fn init_tracing() {
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt::layer().with_target(true))
        .init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    info!("Application initialized successfully !!!");
    let config_path = Path::new(".env");
    let config = if config_path.exists() {
        info!("Loading config from {}", config_path.display());
        AppConfig::from_env_file(config_path)?
    } else {
        error!("No .env file found, using environment variables");
        AppConfig::from_env()?
    };
    info!("Config url: {:?}", config.db_url_std);
    Ok(())
}
