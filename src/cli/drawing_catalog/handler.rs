use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::domain::drawing_catalog::{
    flow::run_catalog,
    model::{SyncDirectoryCommand, SyncOutcome},
};
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

pub async fn run(state: &AppState) -> AppResult<()> {
    info!("====================Drawing====================");
    let total_start = Instant::now();
    let cmd = SyncDirectoryCommand {
        target_dir: PathBuf::from(r"\\192.168.2.219\General\Drawing\Release For Production\"),
    };

    match run_catalog(state, cmd).await {
        Ok(SyncOutcome::Skipped { reason }) => {
            println!("Sync skipped: {}", reason);
            // Or return HTTP 200: {"status": "skipped", "message": reason}
        }
        Ok(SyncOutcome::Completed(summary)) => {
            println!("Sync finished! Inserted: {}", summary.items_inserted);
            // Or return HTTP 201: {"status": "completed", "inserted": ...}
        }
        Err(e) => {
            eprintln!("Error during sync: {}", e);
            // Or return HTTP 500
        }
    }
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END Drawing===================="
    );
    Ok(())
}
