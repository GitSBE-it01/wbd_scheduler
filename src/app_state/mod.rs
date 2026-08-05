use crate::app_config::AppConfig;
use crate::app_error::AppError;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::time::Duration;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool, // No need for Arc
    pub config: AppConfig,
}
impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        info!("Initializing database connection pool...");
        let pool = Self::build_pool(&config).await?;
        Self::test_connection(&pool).await?;
        info!("Database connection pool ready");
        Ok(Self { pool, config })
    }

    async fn build_pool(config: &AppConfig) -> Result<MySqlPool, AppError> {
        let options = MySqlPoolOptions::new()
            .max_connections(3)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60)); // Recycle idle connections
        options.connect(&config.db_url_std).await.map_err(|e| {
            error!("Database connection failed: {}", e);
            AppError::DatabaseConnection(e)
        })
    }

    async fn test_connection(pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query("SELECT 1").fetch_one(pool).await.map_err(|e| {
            error!("Database health check failed: {}", e);
            AppError::DatabaseHealthCheck(e)
        })?;

        Ok(())
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
