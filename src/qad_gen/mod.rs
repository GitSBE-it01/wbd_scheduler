pub mod item_data;
use crate::{app_error::AppResult, app_state::AppState};
use std::time::Instant;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("====================QAD Cache====================");
    let total_start = Instant::now();
    item_data::run(state).await?;
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END QAD Cache===================="
    );
    Ok(())
}
