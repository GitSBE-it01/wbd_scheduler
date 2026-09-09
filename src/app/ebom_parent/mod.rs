use crate::core::ebom_parent::model::EBomOutcome;
use crate::core::ebom_parent::run_ebom;
use crate::{app::error::AppError, system::state::AppState};
use tracing::{error, info, warn};

pub async fn handler(state: &AppState) -> Result<(), AppError> {
    match run_ebom(state).await {
        Ok(EBomOutcome::Skipped { reason }) => {
            warn!("EBom skipped: {}", reason);
        }
        Ok(EBomOutcome::Completed(summary)) => {
            info!(
                base= ?summary.base,
                join_all= ?summary.join_all,
                join_active= ?summary.join_active,
                join_pr_all= ?summary.join_pr_all,
                join_pr_active= ?summary.join_pr_active,
                "EBom finished!");
        }
        Err(e) => {
            error!("Error during running EBOM: {}", e);
        }
    }
    Ok(())
}
