use crate::app_config::AppConfig;
use crate::app_error::{AppError, AppResult};
use sqlx::ConnectOptions;
use sqlx::{
    MySqlPool,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
};
use std::time::Duration;
use tracing::log::LevelFilter;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool, // No need for Arc
    pub config: AppConfig,
}
impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        info!("Initializing database connection pool...");
        let pool = Self::build_pool(&config).await?;
        Self::test_connection(&pool).await?;
        info!("Database connection pool ready");
        Ok(Self { pool, config })
    }

    async fn build_pool(config: &AppConfig) -> AppResult<MySqlPool> {
        let mut connect_options: MySqlConnectOptions = config.db_url_std.parse()?;

        // OPTION A: Increase threshold to 10 seconds (change Duration as needed)
        connect_options =
            connect_options.log_slow_statements(LevelFilter::Warn, Duration::from_secs(60));
        let pool = MySqlPoolOptions::new()
            .max_connections(3)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect_with(connect_options)
            .await?;

        Ok(pool)
    }

    async fn test_connection(pool: &MySqlPool) -> AppResult<()> {
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
