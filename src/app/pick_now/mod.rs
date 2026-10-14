use crate::core::pick_now::model::PnOutcome;
use crate::core::pick_now::pn_handler;
use crate::{app::error::AppError, system::state::AppState};
use tracing::{error, info, warn};

pub async fn run_pick_now(state: &AppState) -> Result<(), AppError> {
    match pn_handler(state).await {
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
