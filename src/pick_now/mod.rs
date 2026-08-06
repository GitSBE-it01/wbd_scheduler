use crate::{app_error::AppResult, app_state::AppState};
use std::time::Instant;
use tracing::info;
mod demand;
mod inventory;
pub mod model;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("Running Pick Now Service...");
    let total_start = Instant::now();
    demand::run(state).await?;
    inventory::run(state).await?;
    let total_dur = total_start.elapsed();
    info!("Pick Now Total duration: {:?}", total_dur);
    Ok(())
}
