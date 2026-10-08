use crate::app::error::AppError;
use crate::core::new_type_report::{flow::run, model::NtrSummary};
use crate::system::state::AppState;
use std::time::Instant;
use tracing::{error, info};

pub async fn run_ntr(state: &AppState) -> Result<(), AppError> {
    match run(state).await {
        Ok(NtrSummary {
            nt_hd_found,
            nt_hd_inserted,
        }) => {
            info!(
                nt_hd_found=?nt_hd_found,
                nt_hd_inserted= ?nt_hd_inserted,
                "Sync finished!",

            );
        }
        Err(e) => {
            error!(?e, "Error during sync");
        }
    }
    Ok(())
}
