pub mod clock;
pub mod error;
pub mod telemetry;
use crate::system::config::AppConfig;
use clock::AppClock;
use error::StateError;
use sqlx::ConnectOptions;
use sqlx::{
    MySqlPool,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
};
use std::time::Duration;
use tracing::info;
use tracing::log::LevelFilter;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: MySqlPool, // No need for Arc
    pub config: AppConfig,
    pub clock: AppClock,
}
impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, StateError> {
        let pool = Self::build_pool(&config).await?;
        Self::test_connection(&pool).await?;
        info!("App State initialized");
        let clock = AppClock::new();
        Ok(Self {
            pool,
            config,
            clock,
        })
    }

    async fn build_pool(config: &AppConfig) -> Result<MySqlPool, StateError> {
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

    async fn test_connection(pool: &MySqlPool) -> Result<(), StateError> {
        sqlx::query("SELECT 1")
            .fetch_one(pool)
            .await
            .map_err(StateError::DatabaseHealthCheck)?;
        Ok(())
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
