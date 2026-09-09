use crate::core::pick_now::model::PnOutcome;
use crate::core::pick_now::run_pick_now;
use crate::{app::error::AppError, system::state::AppState};
use tracing::{error, info, warn};

pub async fn handler(state: &AppState) -> Result<(), AppError> {
    match run_pick_now(state).await {
        Ok(PnOutcome::Skipped { reason }) => {
            warn!("Sync skipped: {}", reason);
        }
        Ok(PnOutcome::Completed(summary)) => {
            info!("Sync finished! Inserted: {}", summary.items_inserted);
        }
        Err(e) => {
            error!("Error during sync: {}", e);
        }
    }
    Ok(())
}
