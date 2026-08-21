use crate::cli::drawing_catalog;
use crate::{
    core::config::AppConfig, core::error::AppResult, core::state::AppState, domain::pick_now,
};
use std::time::Instant;
use tracing::info;

pub async fn main_run() -> AppResult<()> {
    info!("====================Scheduler====================");
    let total_start = Instant::now();
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    drawing_catalog::handler::run(&state).await?;
    pick_now::run(&state).await?;
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Scheduler===================="
    );
    Ok(())
}
