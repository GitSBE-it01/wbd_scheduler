use crate::app::error::AppError;
use crate::core::drawing_catalog::model::SyncOutcome;
use crate::core::jig_catalog::{jig_drawing_handle, model::ScanDirectoryCommand};
use crate::system::state::AppState;
use std::path::PathBuf;
use tracing::{error, info, warn};

pub async fn run_jig_catalog(state: &AppState) -> Result<(), AppError> {
    let cmd = ScanDirectoryCommand {
        target_dir: vec![
            PathBuf::from(r"\\192.168.0.203\Fabrication Tooling Spec\Release for Production\"),
            PathBuf::from(r"\\192.168.2.219\General\Drawing\Release For Production\"),
        ],
    };

    match jig_drawing_handle(state, cmd).await {
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
