use crate::{app_error::AppResult, app_state::AppState};
use std::time::Instant;
use tracing::info;
mod demand;
pub mod model;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("Running Pick Now Service...");
    let total_start = Instant::now();
    let dmd = demand::run(state).await?;
    let total_dur = total_start.elapsed();
    info!("Pick Now Total duration: {:?}", total_dur);
    Ok(())
}
