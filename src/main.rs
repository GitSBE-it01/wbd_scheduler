use tracing::info;
use wbd_scheduler::{
    app_config::AppConfig, app_error::AppResult, app_state::AppState, init_tracing, pick_now,
};

#[tokio::main]
async fn main() -> AppResult<()> {
    init_tracing();
    info!("Initialized Tracing");
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    info!("Initialized State Done");
    pick_now::run(&state).await?;
    Ok(())
}
