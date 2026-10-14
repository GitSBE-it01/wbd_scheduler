use crate::app::error::AppError;
use crate::core::drawing_catalog::{
    dc_handler,
    model::{SyncDirectoryCommand, SyncOutcome},
};
use crate::system::state::AppState;
use std::path::PathBuf;
use tracing::{error, info, warn};

pub async fn run_drawing_catalog(state: &AppState) -> Result<(), AppError> {
    let cmd = SyncDirectoryCommand {
        target_dir: PathBuf::from(r"\\192.168.2.219\General\Drawing\Release For Production\"),
    };

    match dc_handler(state, cmd).await {
        Ok(SyncOutcome::Skipped { reason }) => {
            warn!("Sync skipped: {}", reason);
        }
        Ok(SyncOutcome::Completed(summary)) => {
            info!("Sync finished! Inserted: {}", summary.items_inserted);
        }
        Err(e) => {
            error!("Error during sync: {}", e);
        }
    }
    Ok(())
}
