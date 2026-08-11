use std::time::Instant;
use tracing::{error, info};
use wbd_scheduler::{
    app_config::AppConfig,
    app_error::AppResult,
    app_state::{AppState, telemetry},
    drawing, pick_now, qad_gen,
};

#[tokio::main]
async fn main() {
    let _guard = telemetry::init_telemetry("./logs", "scheduler.log");
    if let Err(err) = main_run().await {
        error!(error = ?err, "Application exited with error");
    }
}

async fn main_run() -> AppResult<()> {
    info!("====================Scheduler====================");
    let total_start = Instant::now();
    let config = AppConfig::new(".env")?;
    let state = AppState::new(config).await?;
    drawing::run(&state).await?;
    qad_gen::run(&state).await?;
    pick_now::run(&state).await?;
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Scheduler===================="
    );
    Ok(())
}
