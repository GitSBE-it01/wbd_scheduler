use dotenvy::from_path;
use serde::Deserialize;
use sqlx::{Connection, MySqlConnection, Row, mysql::MySqlRow};
use std::path::Path;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub db_url_std: String,
    pub db_url_new: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("./.env");
        if config_path.exists() {
            info!("Loading configuration from {}", config_path.display());
            from_path(config_path)?;
        } else {
            error!(".env file not found, attempting to read environment variables directly");
        }

        let config = envy::from_env::<AppConfig>()?;
        Ok(config)
    }
}

fn init_tracing() {
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt::layer().with_target(true))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let config = AppConfig::load().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;
    info!("Configuration loaded successfully: {:?}", config.db_url_std);
    // 3. Connect to MySQL using configured URL instead of hardcoded string
    let mut conn = MySqlConnection::connect(&config.db_url_std).await?;
    info!("Successfully connected to MySQL database");

    let row: MySqlRow = sqlx::query("SELECT * FROM db_wbd.wbd_test WHERE id = 1")
        .fetch_one(&mut conn)
        .await?;

    // 3. Extract the column value using .get() with explicit type binding
    let sum: i64 = row.get("id");
    println!("id= {}", sum);

    Ok(())
}
