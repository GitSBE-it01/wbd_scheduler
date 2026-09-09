use crate::app::error::AppResult;
use crate::system::state::AppState;
use crate::core::new_type_report::{flow::run, model::NtrSummary};
use std::time::Instant;
use tracing::info;

pub async fn run_ntr(state: &AppState) -> AppResult<()> {
    info!("====================NTR====================");
    let total_start = Instant::now();
    match run(state).await {
        Ok(NtrSummary {
            nt_hd_found,
            nt_hd_inserted,
        }) => {
            println!(
                "Sync finished! Found: {}, Inserted: {}",
                nt_hd_found, nt_hd_inserted
            );
        }
        Err(e) => {
            eprintln!("Error during sync: {}", e);
        }
    }
    let total_dur = total_start.elapsed();
    info!(
        duration = ?total_dur,
        "====================END NTR===================="
    );
    Ok(())
}
